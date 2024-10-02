const socket = new WebSocket('ws://192.168.1.169:3032/ws');

console.log('hello from js!');

socket.onmessage = function(event) {

    console.log('Message from server:', event.data);

    let jsonObj = JSON.parse(event.data);

    document.getElementById('board-name').innerHTML = jsonObj.board_name;

    for(let current_status = 0; current_status < jsonObj.statuses.length; current_status++) {

        let status = jsonObj.statuses[current_status];

        let col_by_id = document.getElementById('status-columns');

        col_by_id.innerHTML += '<div id="status-' + status + '">'+status+'<div id = "item-box-'+status+'"></div>';

        for (let current_item = 0; current_item < jsonObj.items.length; current_item++) {

            let item = jsonObj.items[current_item];

            if (item.status == status) {
                console.log(item.name + ' had status ' + status);
                let item_box_by_id = document.getElementById('item-box-'+status);
                item_box_by_id.innerHTML += item.name + '<br>';
            }
        }
    }

    document.getElementById('server-output').innerHTML += event.data + ' </br>';

};

socket.onopen = function() {

    const onopen_response = {
        value: 'ready'
    }

    let req_as_json = JSON.stringify(onopen_response);

    socket.send(req_as_json);
};

socket.onerror = function(error) {
    console.error('WebSocket error:', error);
};

socket.onclose = function() {
    console.log('WebSocket connection closed');
};