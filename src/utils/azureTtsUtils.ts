import * as sdk from 'microsoft-cognitiveservices-speech-sdk';
import { fetch } from "@tauri-apps/plugin-http";

export interface Speaker {
    shortName: string;
    localName: string;
}

const Region = 'eastus';

export const generateCompleteAudioData = async (text: string): Promise<Uint8Array> => {
    const segments = splitTextIntoSegments(text); // 使用你的拆分方法
    const audioDataArray: Uint8Array[] = [];

    for (const segment of segments) {
        const audioData = await generateAudioData(segment);
        audioDataArray.push(audioData);
    }

    // 合并所有音频片段
    const totalLength = audioDataArray.reduce((acc, audioData) => acc + audioData.length, 0);
    const completeAudioData = new Uint8Array(totalLength);
    let offset = 0;

    for (const audioData of audioDataArray) {
        completeAudioData.set(audioData, offset);
        offset += audioData.length;
    }

    return completeAudioData;
}

export const generateAudioData = async (text: string): Promise<Uint8Array> => {
    const speaker = localStorage.getItem('azureTtsSpeaker');
    const rate = parseFloat(localStorage.getItem('azureTtsRate') ?? '1');
    const key = localStorage.getItem('azureTtsKey');
    if (!key) {
        throw new Error('Azure TTS密钥未设置');
    }
    if (!speaker) {
        throw new Error('Azure TTS说话人未设置');
    }

    const speechConfig = sdk.SpeechConfig.fromSubscription(key, Region);
    speechConfig.speechSynthesisVoiceName = speaker;
    const audioOutputStream = sdk.AudioOutputStream.createPullStream();
    const audioConfig = sdk.AudioConfig.fromStreamOutput(audioOutputStream);
    const synthesizer = new sdk.SpeechSynthesizer(speechConfig, audioConfig);

    const ssml = `
    <speak version="1.0" xmlns="http://www.w3.org/2001/10/synthesis" xml:lang="zh-CN">
        <voice name="${speechConfig.speechSynthesisVoiceName}">
            <prosody rate="${rate}">${text}</prosody>
        </voice>
    </speak>`;

    return new Promise((resolve, reject) => {
        synthesizer.speakSsmlAsync(ssml,
            (result) => {
                if (result.reason === sdk.ResultReason.SynthesizingAudioCompleted) {
                    const audioData = new Uint8Array(result.audioData);
                    resolve(audioData);
                } else {
                    reject(new Error(`语音合成失败: ${result.errorDetails}`));
                }
                synthesizer.close();
            },
            (err) => {
                reject(new Error(`错误: ${err}`));
                synthesizer.close();
            });
    });
}


export const getSpeakers = async (): Promise<Speaker[]> => {
    const key = localStorage.getItem('azureTtsKey');
    if (!key) {
        throw new Error('Azure TTS密钥未设置');
    }

    const endpoint = `https://${Region}.tts.speech.microsoft.com/cognitiveservices/voices/list`;

    try {
        const response = await fetch(endpoint, {
            method: 'GET',
            headers: {
                'Ocp-Apim-Subscription-Key': key
            }
        });

        if (!response.ok) {
            throw new Error(`获取说话人列表失败，状态码: ${response.status}`);
        }

        const voices = await response.json();
        console.log(voices);
        const zhVoices = voices.filter((voice: { Locale: string; }) => voice.Locale === 'zh-CN');
        const voiceDetails: Speaker[] = zhVoices.map((voice: { ShortName: string; LocalName: string; }) => ({
            shortName: voice.ShortName,
            localName: voice.LocalName
        }));

        return voiceDetails;
    } catch (error) {
        throw error;
    }
}


export const splitTextIntoSegments = (text: string, maxSegmentLength: number = 3000): string[] => {
    const sentenceEndings = /(?<=[。！？\.\!\?])/g;
    const segments: string[] = [];
    let currentSegment = '';

    const sentences = text.split(sentenceEndings);

    for (const sentence of sentences) {
        if ((currentSegment + sentence).length > maxSegmentLength) {
            segments.push(currentSegment);
            currentSegment = sentence;
        } else {
            currentSegment += sentence;
        }
    }
    if (currentSegment) {
        segments.push(currentSegment);
    }
    return segments;
}