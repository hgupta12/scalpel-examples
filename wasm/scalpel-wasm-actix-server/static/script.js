import init, {dissect_packet} from './pkg/scalpel_wasm_example.js'

const form = document.querySelector('form')

const handleSubmit = async (e) => {
    e.preventDefault();
    const form = e.currentTarget;
    const url = new URL(form.action)
    const formData = new FormData(form);

    const fetchOptions = {
        method: form.method,
        body: formData,
      };

    const req = await fetch(url, fetchOptions);
    if(req.status == 200){
      let data = await req.json();
      showList(data)
    } else {
      let error = await req.json();
      alert(error.error)
    }
}

async function initialize() {
  await init();
}

function formatUnixTimestamp(timestamp) {
  const date = new Date(timestamp * 1000)
  return date.toLocaleString()

}

function showList(data){
  const list = document.querySelector('#list')
  list.innerHTML = ''
  data.forEach((packet, index) => {
    const li = document.createElement('li')
    li.classList.add('list-item')
    const num = document.createElement('p')
    const timestamp = document.createElement('p')
    const len = document.createElement('p')
    num.textContent = `Packet ${index + 1}`
    timestamp.textContent = `Timestamp - ${formatUnixTimestamp(packet.timestamp)}`
    len.textContent =  `Length - ${packet.len}`
    li.appendChild(num)
    li.appendChild(timestamp)
    li.appendChild(len)
    list.appendChild(li)
    li.addEventListener('click', () => generateTableFromJSON(JSON.parse(dissect_packet(packet.data)).layers))
  })

}

function generateTableFromJSON(jsonObj) {
  var container = document.getElementById('packet');
  container.innerHTML = ''
  for (var key in jsonObj) {
    var table = document.createElement('table');
    var header = document.createElement('h2');
    header.textContent = key.toUpperCase();
    container.appendChild(header);
    container.appendChild(table);
    var tbody = document.createElement('tbody');
    table.appendChild(tbody);
    for (var prop in jsonObj[key]) {
      var tr = document.createElement('tr');
      var th = document.createElement('th');
      th.textContent = prop.replace(/_/g, ' ').toUpperCase();
      var td = document.createElement('td');
      td.textContent = jsonObj[key][prop];
      tr.appendChild(th);
      tr.appendChild(td);
      tbody.appendChild(tr);
    }
  }
}

window.onload = initialize

form.addEventListener('submit', handleSubmit)