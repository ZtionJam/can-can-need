<template>
  <div class="main" @click="active = {}">
    <div class="log-box">
      <div class="title">实况</div>
      <img :src="img_now" alt="CanCanNeed" />
    </div>
    <div class="set-box">
      <div class="set-item" v-for="(item, index) in settingList" :key="index">
        <div class="label">当出现【{{ index == "one" ? "1" : index == "two" ? "2" : "3" }}】个人时：</div>
        <div class="item-row" v-for="(item2, index2) in item" :key="index2">
          <div class="row-label">操作应用</div>
          <span class="tip">应用名:</span><input v-model="item2.name" /> <span class="tip">操作:</span
          ><select v-model="item2.type">
            <option value="0">关闭</option>
            <option value="1">最小化</option>
          </select>
          <div class="plus" @click="addItem(index)">+</div>
        </div>
      </div>
    </div>
    <div class="btn-box">
      <div class="btn" @click="save">保存</div>
      <div class="btn" @click="openCam">实况</div>
    </div>
  </div>
</template>

<script setup>
import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";

const img_now = ref("https://img2.baidu.com/it/u=2778168081,4108854712&fm=253&fmt=auto&app=138&f=JPEG?w=335&h=245");
const settingList = ref({
  one: [{}],
  two: [{}],
  three: [{}],
});

const active = ref({});
const openCam = () => {
  invoke("link_start");
};

listen("update_img", (event) => {
  img_now.value = "data:image/png;base64," + event.payload;
});

function addItem(index) {
  settingList.value[index].push({});
}

function save() {
  console.log(settingList.value);
}
</script>

<style>
* {
  padding: 0;
  margin: 0;
  box-sizing: border-box;
  -webkit-user-drag: none;
}

input {
  background: #fff;
  border: 1px solid #b2b2b2;
  border-radius: 0;
  font-size: 13px;
}
.tip {
  font-size: 12px;
  margin-left: 10px;
  margin-right: 5px;
}

input[readonly] {
  cursor: text;
}
</style>

<style lang="scss" scoped>
.main {
  width: 100%;
  height: 100vh;
  display: flex;
  flex-direction: column;

  .log-box {
    width: 100%;
    border: 1px solid #66666644;
    margin-bottom: 20px;
    border-radius: 10px;
    padding: 5px;

    .title {
      width: 100%;
      text-align: center;
      font-size: 16px;
      font-weight: bold;
      margin-bottom: 10px;
      color: #999;
      position: sticky;
      top: 0px;
    }

    img {
      margin-right: 10px;
      border: 1px solid red;
      width: 100%;
    }

    .text {
      flex: 1;
      font-size: 15px;
      margin-bottom: 5px;
      color: #666666;
    }
  }
}

.set-box {
  width: 100%;
  //min-height: 200px;
  display: flex;
  flex-direction: column;
  justify-content: space-between;

  .set-item {
    border-radius: 10px;
    padding: 5px 10px;
    border: 1px solid #66666650;

    min-height: 100px;

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
        width: 200px;
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
