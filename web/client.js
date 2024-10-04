function add_new_item() {

    console.log('todo: add new item');

    let input_name = document.getElementById('item-name-input').value;
    let input_content = document.getElementById('item-content-input').value;

    if (input_name != '' && input_content != '') {
        const add_item_response = {
            command: 'add',
            args: [input_name, input_content],
        }
        let req_as_json = JSON.stringify(add_item_response);
        console.log('sending: ' + req_as_json);
        socket.send(req_as_json);
    } else {
        console.log('handle empty fields complaint');
    }
}

var htmlString = `
    <div id = "header-div"><h1 id = "board-name"></h1></div>
    <div class = "status-columns" id = "status-columns"></div>
    <br>
    <br>
    Item Name: <input type="text" name="item-name-input-box" id="item-name-input"> Item Content: <input type="text" name="item-content-input-box" id="item-content-input">
    <button type="button" id="add-button">Add it!</button>
    <br>
    <br>
    <div id = "server-output"></div>
    `;

function build_board(response) {
    let parser = new DOMParser();
    const doc = parser.parseFromString(htmlString, 'text/html');
    doc.getElementById('add-button').innerHTML = 'Add it?';
    doc.getElementById('board-name').innerHTML = response.board_name;

    for(let current_status = 0; current_status < response.statuses.length; current_status++) {

        let status = response.statuses[current_status];

        let col_by_id = doc.getElementById('status-columns');

        col_by_id.innerHTML += '<div id="status-' + status + '">'+status.toUpperCase()+'<div id = "item-box-'+status+'"></div>';

        for (let current_item = 0; current_item < response.items.length; current_item++) {

            let item = response.items[current_item];

            if (item.status == status) {
                console.log(item.name + ' had status ' + status);
                let item_box_by_id = doc.getElementById('item-box-'+status);
                item_box_by_id.innerHTML +=
                    '<div id="item-cont">'+
                        '<div id="item-name">'+
                            '<div id="name-container">'+ item.name + '</div>'+
                            '<div id ="demote-box" class="'+item.name+'">↓</div>'+
                            '<div id ="promote-box" class="'+item.name+'">↑</div>'+
                            '<div id ="edit-content-box" class="'+item.name+'">✎</div>'+
                            '<div id ="remove-item-box" class="'+item.name+'">✖</div>'+
                        '</div>'+
                        '<div id="item-contents">'+item.contents + '</div></div>'+
                    '</div>';
            }
        }
    }

    doc.getElementById('server-output').innerHTML += event.data + ' </br>';

    document.getElementsByTagName('body')[0].innerHTML = doc.documentElement.outerHTML;
    
}

console.log('starting web socket...');

const socket = new WebSocket('ws://192.168.1.169:3032/ws');

socket.onmessage = function(event) {
    console.log('incoming:', event.data);
    build_board(JSON.parse(event.data));
    document.getElementById('add-button').addEventListener("click", add_new_item);
};
socket.onopen = function() {

    const ready_response = {
        command: 'ready',
        args: [],
    }

    let req_as_json = JSON.stringify(ready_response);
    console.log('ready; sending: ' + req_as_json);
    socket.send(req_as_json);
};

socket.onerror = function(error) {
    console.error('WebSocket error:', error);
};

socket.onclose = function() {
    console.log('WebSocket connection closed');
};