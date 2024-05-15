<script setup>
import  A from "./components/A.vue";
// 禁用触摸板拖动的函数
function disableDrag(e) {
  e.preventDefault();
}

// 页面加载时绑定事件
window.onload = function() {
  // 添加事件监听到文档，用于阻止拖动事件
  document.addEventListener('touchmove', disableDrag, { passive: false });
};
</script>

<template>
 
   <A/>

</template>

<style>
*{
  padding: 0;
  margin: 0;
}
::-webkit-scrollbar {
  width: 0;
}
</style>
