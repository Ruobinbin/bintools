import { invoke } from "@tauri-apps/api/core";
import { parseCookies } from "./defaultUtils";

interface wsPacket {
    Len?: number;
    HeadLen?: number;
    Ver?: number;
    Type?: number;
    Num?: number;
    body?: string | number;
}

function randomElement<T>(list: T[]): T {
    return list[Math.floor(Math.random() * list.length)]
}

export const webSocket = async (room_id: number, cookie?: string): Promise<void> => {
    if ("WebSocket" in window) {
        console.log("您的浏览器支持WebSocket");
        var timer: number | null = null;
        const room = await invoke<any>('proxy_request', { url: `https://api.live.bilibili.com/room/v1/Room/get_info?room_id=${room_id}` })
            .then(res => res)
            .catch(err => {
                console.log(`获取房间ID失败: ${err as string}`);
                return;
            });
        const damuInfoUrl = `https://api.live.bilibili.com/xlive/web-room/v1/index/getDanmuInfo?id=${room.data.room_id}`;
        let danmuInfo: any;
        let uid: number;
        let buvid: string;

        if (cookie) {
            const cookies = parseCookies(cookie);
            uid = parseInt(cookies.DedeUserID);
            buvid = cookies.buvid3;
            await invoke<any>('proxy_request', { url: damuInfoUrl, cookie: cookie })
                .then(response => {
                    danmuInfo = response;
                })
                .catch(err => {
                    console.log(`获取弹幕服务器配置失败: ${err as string}`);
                    return;
                });
        } else {
            await invoke<any>('proxy_request', { url: damuInfoUrl })
                .then(response => {
                    danmuInfo = response;
                })
                .catch(err => {
                    console.log(`获取弹幕服务器配置失败: ${err as string}`);
                    return;
                });
            uid = 0;
            buvid = "";
        }

        const host: any = randomElement(danmuInfo.data.host_list)
        const url = `wss://${host.host}:${host.wss_port}/sub`

        var certification = {
            "uid": uid,
            "roomid": room.data.room_id,
            "protover": 3,
            "buvid": buvid,
            "platform": "web",
            "type": 2,
            "key": danmuInfo.data.token  //值为空字符串好像也没问题
        }
        console.log(JSON.stringify(certification))

        var ws = new WebSocket(url);

        ws.onopen = function (e) {
            console.log("open");
            ws.send(getCertification(JSON.stringify(certification)));
            //发送心跳包
            timer = setInterval(function () {
                let buff = new ArrayBuffer(16);
                let i = new DataView(buff);
                i.setUint32(0, 16);    //整个封包
                i.setUint16(4, 16);    //头部
                i.setUint16(6, 1);    //协议版本
                i.setUint32(8, 2);    //操作码,2为心跳包
                i.setUint32(12, 1);    //填1
                ws.send(buff);
            }, 30000); //30秒

        }

        ws.onmessage = function (e) {
            let blob = e.data;
            let decoder = new TextDecoder();
            let reader = new FileReader();
            reader.readAsArrayBuffer(blob);
            reader.onload = async function () {
                let buff = reader.result as ArrayBuffer;
                let view = new DataView(buff);
                let offset = 0;
                let packet: wsPacket = {};
                // let result: wsPacket[] = [];
                while (offset < buff.byteLength) {
                    let packetLen = view.getUint32(offset + 0);
                    let headLen = view.getUint16(offset + 4);
                    let packetVer = view.getUint16(offset + 6);
                    let packetType = view.getUint32(offset + 8);
                    let num = view.getUint32(12);
                    let dataArray = new Uint8Array(buff, offset + headLen, packetLen - headLen);
                    if (packetVer === 3) {
                        await invoke<Uint8Array>('decompress_brotli', { data: dataArray }).then(data => {
                            let uint8Array = new Uint8Array(data);
                            let arrayBuffer = uint8Array.buffer;
                            let view = new DataView(arrayBuffer);
                            let offset_Ver3 = 0;
                            while (offset_Ver3 < arrayBuffer.byteLength) {
                                let packetLen = view.getUint32(offset_Ver3 + 0);
                                let headLen = view.getUint16(offset_Ver3 + 4);
                                let packetVer = view.getUint16(offset_Ver3 + 6);
                                let packetType = view.getUint32(offset_Ver3 + 8);
                                let num = view.getUint32(12);
                                packet.Len = packetLen;
                                packet.HeadLen = headLen;
                                packet.Ver = packetVer;
                                packet.Type = packetType;
                                packet.Num = num;
                                let dataArray = new Uint8Array(arrayBuffer, offset_Ver3 + headLen, packetLen - headLen);
                                packet.body = JSON.parse(decoder.decode(dataArray));
                                offset_Ver3 += packetLen;
                            }
                        }).catch(err => {
                            console.error('解压数据失败:', err);
                            return new Uint8Array();
                        })
                    } else {
                        packet.Len = packetLen;
                        packet.HeadLen = headLen;
                        packet.Ver = packetVer;
                        packet.Type = packetType;
                        packet.Num = num;
                        if (packetType == 3) {
                            packet.body = (new DataView(buff, offset + headLen, packetLen - headLen)).getUint32(0);
                        } else {
                            packet.body = JSON.parse(decoder.decode(dataArray));
                        }
                    }
                    console.log(packet.body)
                    offset += packetLen;
                }


            }
            // handleMessage(blob, function (result: string[]) {
            //     //触发事件
            //     for (let i = 0; i < result.length; i++) {
            //         let json = JSON.parse(result[i]);
            //         if (json.Type == 5) {
            //             let event = new CustomEvent(JSON.parse(json.body).cmd, { detail: JSON.parse(json.body) });
            //             eventTarget.dispatchEvent(event);
            //         }
            //         if (json.Type == 8) {
            //             let event = new CustomEvent("Certify_Success", { detail: JSON.parse(json.body) });
            //             eventTarget.dispatchEvent(event);
            //         }
            //         if (json.Type == 3) {
            //             let event = new CustomEvent("VIEW", { detail: json.body });
            //             eventTarget.dispatchEvent(event);
            //         }
            //     }
            // });
        }

        ws.onclose = function (e) {
            //当客户端收到服务端发送的关闭连接请求时，触发onclose事件
            console.log("close");
            if (timer != null) {
                clearInterval(timer);    //停止发送心跳包
            }
            // setTimeout(() => webSocket(room_id), 4000);    //4秒后重连
        }

        ws.onerror = function (e) {
            //如果出现连接、处理、接收、发送数据失败的时候触发onerror事件
            console.log(e);
        }
    } else {
        console.log("您的浏览器不支持WebSocket");
    }
}

//生成认证数据
const getCertification = (json: string): ArrayBuffer => {
    let encoder = new TextEncoder();    //编码器
    let jsonView = encoder.encode(json);    //utf-8编码
    let buff = new ArrayBuffer(jsonView.byteLength + 16);    //数据包总长度：16位头部长度+bytes长度
    let view = new DataView(buff);    //新建操作视窗
    view.setUint32(0, jsonView.byteLength + 16);    //整个数据包长度
    view.setUint16(4, 16);    //头部长度
    view.setUint16(6, 1);    //协议版本
    view.setUint32(8, 7);    //类型,7为加入房间认证
    view.setUint32(12, 1);    //填1
    for (let r = 0; r < jsonView.byteLength; r++) {
        view.setUint8(16 + r, jsonView[r]);    //填入数据
    }
    return buff;
}

