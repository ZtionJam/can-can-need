<template>
  <div class="main" @click="active = {}">
    <div class="log-box">
      <div class="title">日志</div>
      <div class="log-item" v-for="item in logList">
        <img :src="item.image" alt="CanCanNeed" />
        <div class="text">{{ item.content }}</div>
      </div>
    </div>
    <div class="set-box">
      <div class="set-item" v-for="(item, index) in settingList" :key="index">
        <div class="label">当出现【{{ index == "one" ? "1" : index == "two" ? "2" : "3" }}】个人时：</div>
        <div class="item-row" v-for="(item2, index2) in item" :key="index2">
          <div class="row-label">触发按键：</div>
          <input :class="{ custom: active.index1 == index && active.index2 == index2 && active.key == 'key1' }" readonly maxlength="1" v-model="item2.key1.label" @click.stop="handleClick(index, index2, 'key1')" @keydown="(e) => handleInput(e, item2, 'key1')" />
          <div class="row-gap">+</div>
          <input :class="{ custom: active.index1 == index && active.index2 == index2 && active.key == 'key2' }" readonly maxlength="1" v-model="item2.key2.label" @click.stop="handleClick(index, index2, 'key2')" @keydown="(e) => handleInput(e, item2, 'key2')" />
          <div class="row-gap">+</div>
          <input :class="{ custom: active.index1 == index && active.index2 == index2 && active.key == 'key3' }" readonly maxlength="1" v-model="item2.key3.label" @click.stop="handleClick(index, index2, 'key3')" @keydown="(e) => handleInput(e, item2, 'key3')" />
          <div class="plus" @click="addItem(index)">+</div>
        </div>
      </div>
    </div>
    <div class="btn-box">
      <div class="btn" @click="save">保存</div>
    </div>
  </div>
</template>

<script setup>
const logList = ref([{ image: "https://img2.baidu.com/it/u=2778168081,4108854712&fm=253&fmt=auto&app=138&f=JPEG?w=335&h=245", content: "日志信息啊是的啊是的萨达" }]);
const settingList = ref({
  one: [
    {
      key1: {},
      key2: {},
      key3: {},
    },
  ],
  two: [
    {
      key1: {},
      key2: {},
      key3: {},
    },
  ],
  three: [
    {
      key1: {},
      key2: {},
      key3: {},
    },
  ],
});

const active = ref({});

function addItem(index) {
  settingList.value[index].push({
    key1: {},
    key2: {},
    key3: {},
  });
}
function save() {
  console.log(settingList.value);
}
function handleInput(e, item, key) {
  item[key].label = e.key;
  item[key].value = e.keyCode;
}
function handleClick(index1, index2, key) {
  active.value.index1 = index1;
  active.value.index2 = index2;
  active.value.key = key;
}
</script>

<style>
* {
  padding: 0;
  margin: 0;
  box-sizing: border-box;
}
input {
  background: #f9f9f988;
  border: 1px solid #eeeeee;
}
input[readonly] {
  cursor: text;
}
</style>

<style lang="scss" scoped>
.main {
  padding: 20px;
  width: 100%;
  height: 100vh;
  display: flex;
  flex-direction: column;
  .log-box {
    width: 100%;
    flex: 1;
    border: 1px solid #66666644;
    margin-bottom: 20px;
    border-radius: 10px;
    padding: 12px;
    overflow: auto;
    position: relative;
    .title {
      width: 100%;
      text-align: center;
      font-size: 20px;
      font-weight: bold;
      margin-bottom: 10px;
      color: #999;
      position: sticky;
      top: 0px;
    }
    .log-item {
      padding: 10px 15px;
      display: flex;
      align-items: center;
      margin-bottom: 5px;
      width: 100%;
      border: 0.5px solid #cccccc;
      border-radius: 10px;
      box-shadow: 5px 5px 8px 0 #666666;
      img {
        width: 150px;
        height: 150px;
        margin-right: 10px;
      }
      .text {
        flex: 1;
        font-size: 15px;
        margin-bottom: 5px;
        color: #666666;
      }
    }
  }
}
.set-box {
  width: 100%;
  min-height: 200px;
  display: flex;
  justify-content: space-between;
  .set-item {
    border-radius: 10px;
    padding: 5px 10px;
    border: 1px solid #66666650;
    flex: 33%;
    flex-grow: unset;
    .label {
      font-size: 16px;
      color: #999;
      font-weight: bold;
    }
    .item-row {
      width: 100%;
      display: flex;
      align-items: center;
      margin-top: 10px;
      .row-label {
        font-size: 13px;
        font-weight: bold;
        color: #333;
      }
      input {
        width: 23%;
      }
      .row-gap {
        margin: 0 1%;
      }
      .plus {
        border-radius: 50%;
        width: 20px;
        height: 20px;
        text-align: center;
        line-height: 20px;
        font-size: 25px;
        background: rgb(117, 190, 255);
        color: #fff;
        margin-left: 2%;
        cursor: pointer;
      }
    }
  }
}
.btn-box {
  margin-top: 15px;
  width: 100%;
  display: flex;
  justify-content: center;
  .btn {
    cursor: pointer;
    width: 60px;
    height: 30px;
    line-height: 30px;
    text-align: center;
    font-size: 15px;
    color: #fff;
    background: rgb(60, 164, 255);
    border-radius: 10px;
  }
}
.custom {
  background: #fff !important;
  border: 1px solid #333;
}
</style>
