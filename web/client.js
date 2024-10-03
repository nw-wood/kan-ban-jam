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

const socket = new WebSocket('ws://192.168.1.169:3032/ws');

console.log('hello from js!');

document.getElementById('add-button').addEventListener("click", add_new_item);

socket.onmessage = function(event) {

    console.log('Message from server:', event.data);

    let jsonObj = JSON.parse(event.data);

    document.getElementById('board-name').innerHTML = jsonObj.board_name;
    //TODO: on message needs to handle server responses as json instead of just assuming it's the entire board now
    for(let current_status = 0; current_status < jsonObj.statuses.length; current_status++) {

        let status = jsonObj.statuses[current_status];

        let col_by_id = document.getElementById('status-columns');

        col_by_id.innerHTML += '<div id="status-' + status + '">'+status.toUpperCase()+'<div id = "item-box-'+status+'"></div>';

        for (let current_item = 0; current_item < jsonObj.items.length; current_item++) {

            let item = jsonObj.items[current_item];

            if (item.status == status) {
                console.log(item.name + ' had status ' + status);
                let item_box_by_id = document.getElementById('item-box-'+status);
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
                    //modal pop up thing with text box input to change the contents of the item
                    //check cli for more
                    //ARE YOU SURE ABOUT THAT? confirmation modal on remove
                    //some kind of first time 'help' dialog and cookie set to prevent it from happening again would be worth spending time on to learn
            }
        }
    }
    document.getElementById('server-output').innerHTML += event.data + ' </br>';
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