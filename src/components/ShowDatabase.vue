<template>
    <div>
        <el-select v-model="selectedTable" placeholder="选择一个表" @change="fetchTableData">
            <el-option v-for="table in tables" :key="table" :label="table" :value="table" />
        </el-select>
        <el-button type="danger" @click="deleteCurrentTable" :disabled="!selectedTable">删除当前表</el-button>
        <el-table :data="tableData" v-if="tableData.length > 0">
            <el-table-column v-for="(key) in Object.keys(tableData[0] || {})" :key="key" :prop="key" :label="key" />
        </el-table>
        <p v-else>没有数据显示</p>
    </div>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue';
import { getTableDataByName, getAllTableNames, deleteTableByName } from '../utils/dbUtils';
import { ElMessageBox } from 'element-plus';

const tables = ref<string[]>([]);
const selectedTable = ref<string>('');
const tableData = ref<Array<Record<string, any>>>([]);

const fetchTables = async () => {
    tables.value = await getAllTableNames();
};

const fetchTableData = async (tableName: string) => {
    if (!tableName) return;
    tableData.value = await getTableDataByName(tableName);
};

const deleteCurrentTable = async () => {
    if (!selectedTable.value) return;

    try {
        await ElMessageBox.confirm(`确定要删除表 "${selectedTable.value}" 吗？`, '警告', {
            confirmButtonText: '确定',
            cancelButtonText: '取消',
            type: 'warning'
        });

        await deleteTableByName(selectedTable.value);
        await fetchTables();
        selectedTable.value = '';
        tableData.value = [];
    } catch (error) {
        console.error('删除表失败:', error);
    }
};

onMounted(() => {
    fetchTables();
});
</script>

<style scoped></style>
