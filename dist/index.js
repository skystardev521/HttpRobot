const { tauri } = require("@tauri-apps/api")
const { listen, emit } = require("@tauri-apps/api/event")


const WebviewWindow = window.__TAURI__.window.WebviewWindow
document.querySelector('#OpenWindow').addEventListener('click', () => {
  new WebviewWindow(Math.random().toString().replace('.', ''), {
    url: "index_bak.html"
  })
})


document.querySelector("#TestCmd").addEventListener("click",()=>{
    const result = document.querySelector("#TestCmdResult")
    window.__TAURI__
        .invoke("simple_command", { argument: 'value' })
        .then((response) => {
          result.innerText = `Ok(${response})`
        })
        .catch((error) => {
          result.innerText = `Err(${error})`
        })

        emit('click', {
            theMessage: 'Tauri is awesome!',
          })
})
      
// window.__TAURI__.event.listen('click', (event) => {
//     // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
//     // event.payload is the payload object
//     const result = document.querySelector("#TestCmdResult")
//     window.__TAURI__
//         .invoke("simple_command", { argument: 'value' })
//         .then((response) => {
//           result.innerText = `Ok(${response})`
//         })
//         .catch((error) => {
//           result.innerText = `Err(${error})`
//         })
//   })

//   window.__TAURI__.event.emit('click1', {
//     theMessage: 'Tauri is awesome!',
//   })

  const unlisten = await listen('click', (event) => {
    const result = document.querySelector("#TestCmdResult")

    result.innerText = `listen click Event`

    // window.__TAURI__
    //     .invoke("simple_command", { argument: 'value1' })
    //     .then((response) => {
    //       result.innerText = `Ok(${response})`
    //     })
    //     .catch((error) => {
    //       result.innerText = `Err(${error})`
    //     })
  });

  emit('click1', {
    theMessage: 'Tauri is awesome!',
  })
