const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;
let json_data = "";

window.addEventListener("DOMContentLoaded", () => {
  const play = document.querySelector("#play");
  const pause = document.querySelector("#pause");
  invoke("check_sources").catch((error) => alert(error));
  Object.values(get_inputs()).forEach(input => {
    input.addEventListener('change', () => {
      save()
    })
  });
  load_data()
  play.addEventListener('click', enable_playing)
  pause.addEventListener('click', () => {
  })
});

var local_files = {}

async function enable_playing() {
  check_inputs()// Проверяем всё ли заполнено
  await get_local_files()
  console.log(local_files)
  //aaa
  //   invoke('get_local_files').then(local_files_json => {
  //     let local_files = JSON.parse(local_files_json); // получаем список локальных файлов
  //     function enable_player(minutes) {
  //       const deadline = new Date(Date.now() + minutes * 60 * 1000); // Вычисляем время окончания проигрывания музыки
  //       const player = document.querySelector(".player")
  //       setInterval(() => {
  //         if (Date.now() <= deadline) {
  //           player.src = `./music/${local_files[0]}`
  //           local_files = local_files.slice(1)
  //         } else {
  //           clearInterval()
  //         }
  //       }, 1000)}
  //       enable_player(0.1)
  //       // while ((Date() - start_playing_date)/1000<5){
  //       //   console.log('abababb')
  //       // }
  //       // цикл{
  //       // чекаем норм ли треком (если не норм то докачиваем)
  //       // меняем в плеере src на первый из актуальных
  //       // стартуем
  //       // ждем окончания
  //       // }
  //     }).catch((error) => alert(error))
}
async function get_local_files() {
  await invoke('get_local_files').then(local_files_json => {
    local_files = JSON.parse(local_files_json)
  }).catch((error) => alert(error))
}
function check_inputs() {
  let inputs = Object.values(get_inputs())
  let neet_alert = false
  inputs.forEach(input => {
    if (input.value == "") {
      neet_alert = true
    }
  })
  if (neet_alert) alert('Не все поля заполнены!');
}

function get_inputs() {
  const ip = document.querySelector("#ip");
  const username = document.querySelector("#username");
  const password = document.querySelector("#password");
  const shopId = document.querySelector("#shop_id");
  const marketingInterval = document.querySelector("#marketing_interval");
  return { 'ip': ip, 'username': username, 'password': password, 'shopId': shopId, 'marketingInterval': marketingInterval }
}

function save(){
  let inputs = get_inputs()
  invoke("save", {
    ip: inputs.ip.value,
    username: inputs.username.value,
    password: inputs.password.value,
    shopId: inputs.shopId.value,
    marketingInterval: inputs.marketingInterval.value
  }).catch((error) => alert(error))
}

function load_data() {
  let inputs = get_inputs()
  invoke("load").then((json_data) => {
    var data = JSON.parse(json_data)
    inputs.ip.value = data.ip;
    inputs.username.value = data.username;
    inputs.password.value = data.password;
    inputs.shopId.value = data['shop_id'];
    inputs.marketingInterval.value = data['marketing_interval']
  }).catch((error) => console.error(error))
}