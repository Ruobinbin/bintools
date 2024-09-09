<template>
    <div>
        <el-select v-model="selectedTable" placeholder="Select a table" @change="fetchTableData">
            <el-option v-for="table in tables" :key="table" :label="table" :value="table" />
        </el-select>
        <el-table :data="tableData" v-if="tableData.length > 0">
            <el-table-column v-for="(key) in Object.keys(tableData[0] || {})" :key="key" :prop="key" :label="key" />
        </el-table>
        <p v-else>No data to display</p>
    </div>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue';
import { getTableDataByName, getAllTableNames } from '../utils/dbUtils'; // 从数据库工具文件导入

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

onMounted(() => {
    fetchTables();
});
</script>

<style scoped></style>
