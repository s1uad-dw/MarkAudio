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
  
  json_data = invoke("load").catch((error) => console.error(error))
  console.log(json_data.data);
  play.addEventListener('click', () => {
    // Проверяем всё ли заполнено
    // Получаем актульнеы треки (смотрим директорию)
    // записываем время старта воспроизведения
    // цикл{
    // меняем в плеере src на первый из актуальных
    // стартуем
    // чекаем норм ли треком (если не норм то докачиваем)
    // ждем окончания
    // }
  })

});
