const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

var inputs = []
window.addEventListener("DOMContentLoaded", () => {
  var ip = document.querySelector("#ip");
  var username = document.querySelector("#username");
  var password = document.querySelector("#password");
  var shop_id = document.querySelector("#shop_id");
  var marketing_interval = document.querySelector("#marketing_interval");

  const play = document.querySelector("#play");
  const pause = document.querySelector("#pause");
  load_data()
  invoke("check_sources").catch((error) => alert(error));
  [ip, username, password, shop_id, marketing_interval].forEach(input => {
    input.addEventListener('change', save)
  });
  player.addEventListener('canplaythrough', function () { player.play() })
  play.addEventListener('click', play_music)
  // pause.addEventListener('click', () => {

  // })
});


var local_files = {}
var player = document.querySelector(".player")
var playing;


async function play_music() {
  check_inputs()// Проверяем всё ли заполнено
  const marketingInterval = document.querySelector("#marketing_interval");
  playing = setTimeout(async function () {//цикол бесконечный
    const deadline = new Date(Date.now() + marketingInterval.value * 60 * 1000);//получаем дедлайн
    await get_local_files()//получаем список локальных файлов
    //чекаем норм ли треков
    if (local_files[0].length < 5) {//не норм
      console.log(local_files[0].length < 5, 6 - local_files[0].length)
      await invoke('download_missing_tracks', {
        ip: ip.value,
        username: username.value,
        password: password.value,
        quantity: 5 - local_files[0].length
      }).catch((error) => alert(error))
      play_music()
    } else {//норм
      player.src = `music/${local_files[0][0]}`//то ставим актуальные src плееру
      setInterval(() => {
        //чекаем не дедлайн ли
        console.log('-------------------------------------')
        console.log(Date.now() > deadline && player.paused)
        console.log(Date.now() < deadline && player.paused)
        console.log(player.src)
        if (Date.now() > deadline && player.paused) {//дедлайнa
          local_files[0] = local_files[0].slice(1)//удаляем трек который поставили в src
          invoke('remove_file', { path: `music/${local_files[0][0]}` }).catch((error) => alert(error))
          clearInterval()
          play_marketing()
          play_music()
          //играем рекламу
        } else if (Date.now() < deadline && player.paused) {
          local_files[0] = local_files[0].slice(1)//удаляем трек который поставили в src
          invoke('remove_file', { path: `music/${local_files[0][0]}` }).catch((error) => alert(error))
          play_music()
        }
      }, 1000)
    }
  })
}

function play_marketing() {
  player.src = `marketing/${local_files[1][0]}`
}

async function get_local_files() {
  await invoke('get_local_files').then(local_files_json => {
    local_files = JSON.parse(local_files_json)
  }).catch((error) => alert(error))
}
function check_inputs() {
  let neet_alert = false
  let inputs = [ip, username, password, shop_id, marketing_interval]
  inputs.forEach(input => {
    if (input.value == "") {
      neet_alert = true
    }
  })
  if (neet_alert) alert('Не все поля заполнены!');
}

function save() {
  invoke("save", {
    ip: ip.value,
    username: username.value,
    password: password.value,
    shopId: shop_id.value,
    marketingInterval: marketing_interval.value
  }).catch((error) => alert(error))
}

function load_data() {
  invoke("load").then((json_data) => {
    var data = JSON.parse(json_data)
    ip.value = data.ip;
    username.value = data.username;
    password.value = data.password;
    shop_id.value = data['shop_id'];
    marketing_interval.value = data['marketing_interval']
  }).catch((error) => console.error(error))
}