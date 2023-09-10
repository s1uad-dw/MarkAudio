const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;
let json_data = "";

window.addEventListener("DOMContentLoaded", () => {
  const ip = document.querySelector("#ip");
  const username = document.querySelector("#username");
  const password = document.querySelector("#password");
  const shopId = document.querySelector("#shop_id");
  const marketingInterval = document.querySelector("#marketing_interval");

  const sync_button = document.querySelector("#sync");

  const player = document.querySelector(".player")
  const play = document.querySelector("#play");
  const pause = document.querySelector("#pause");

  invoke("check_sources").catch((error) => alert(error));
  function load_data() {
    invoke("load").then((json_data) => {
      var data = JSON.parse(json_data)
      ip.value = data.ip;
      username.value = data.username;
      password.value = data.password;
      shopId.value = data['shop_id'];
      marketingInterval.value = data['marketing_interval']
    }).catch((error) => console.error(error))
  }

  function check_inputs() {
    let inputs = [ip, username, password, shopId, marketingInterval]
    let neet_alert = false
    inputs.forEach(input => {
      if (input.value == "") {
        neet_alert = true
      }
    })
    if (neet_alert) {
      alert('Не все поля заполнены!');
    }
  }
  [ip, username, password, shopId, marketingInterval].forEach(input => {
    input.addEventListener('change', () => {
      invoke("save", {
        ip: ip.value,
        username: username.value,
        password: password.value,
        shopId: shopId.value,
        marketingInterval: marketingInterval.value,
        recentTracks: NaN
      }).catch((error) => alert(error))
    })
  });


  load_data()

  play.addEventListener('click', () => {

    check_inputs()// Проверяем всё ли заполнено
    invoke('get_local_files').then(local_files_json => {
      let local_files = JSON.parse(local_files_json); // получаем список локальных файлов
      invoke('write_time').catch((error) => alert(error))
      // while ((Date() - start_playing_date)/1000<5){
      //   console.log('abababb')
      // }
      // цикл{
      // чекаем норм ли треком (если не норм то докачиваем)
      // меняем в плеере src на первый из актуальных
      // стартуем
      // ждем окончания
      // }
    }).catch((error) => alert(error))
    
    
  })
  pause.addEventListener('click', () => {
  })
});