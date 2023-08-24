<script setup lang="ts">
import {reactive, ref} from 'vue'


const name = ref('')
let loginDate = reactive({
  name: "",
  password: "",
})

function sub_btn() {
  console.log(loginDate)
  // router.push();
}
</script>

<template>
  <div class="login">
    <h3>QQ Home</h3>
    <el-form
        label-width="80px"
        :model="loginDate">
      <el-form-item label="账号">
        <el-input placeholder="请输入账号" v-model="loginDate.name"></el-input>
      </el-form-item>
      <el-form-item label="密码">
        <el-input placeholder="请输入密码" v-model="loginDate.password" show-password></el-input>
      </el-form-item>
      <el-form-item>
        <el-button @click="sub_btn" class="btn_sub" type="primary">登录</el-button>
      </el-form-item>
    </el-form>
  </div>
</template>

<style scoped>
.login {
  width: 450px;
  margin: 10px auto;
  border: 1px solid #efefef;
  padding: 20px;
  border-radius: 10px;
}

h4 {
  text-align: center;
}

.btn_sub {
  position: center;
}
</style>





