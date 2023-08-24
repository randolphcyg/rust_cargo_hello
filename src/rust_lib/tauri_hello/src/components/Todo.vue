<script setup lang="ts">
import {ref} from "vue";
import {Close} from '@element-plus/icons-vue'

let id = 0
const todosList = ref([])
const text = ref('')

const onAddTodo = () => {
  todosList.value.push({
    id: id++,
    text: text.value
  })
}

const onComplete = index => {
  todosList.value.splice(index, 1)
}
</script>

<template>
  <div class="todo-container">
    <h1>Todo List</h1>
    <div class="head">
      <el-input type="text" v-model="text"></el-input>
      <el-button @click="onAddTodo" type="primary">添 加</el-button>
    </div>
    <div class="list" v-if="todosList.length">
      <ul>
        <li v-for="(todo, index) in todosList" :key="todo.id">
          <span>{{ index + 1 }}. </span>
          <span>{{ todo.text }}</span>
          <el-icon :size="16" color="gray" @click="onComplete(index)" style="cursor: pointer;">
            <Close/>
          </el-icon>
        </li>
      </ul>
    </div>
  </div>
</template>



