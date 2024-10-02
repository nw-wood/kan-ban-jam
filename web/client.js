const socket = new WebSocket('ws://192.168.1.169:3032/ws');

console.log('hello from js!');
// Listen for messages from the server
socket.onmessage = function(event) {
    console.log('Message from server:', event.data);

    //handle responses from the server here!

    let jsonObj = JSON.parse(event.data);

    //update the name for the board on the page
    document.getElementById('board-name').innerHTML = jsonObj.board_name;

    for(let current_status = 0; current_status < jsonObj.statuses.length; current_status++) {
        let status = jsonObj.statuses[current_status];
        for (let current_item = 0; current_item < jsonObj.items.length; current_item++) {
            let item = jsonObj.items[current_item];
            //console.log(item);
            //if item.status == status
        }
        document.getElementById('status-columns').innerHTML = document.getElementById('status-columns').innerHTML + '<div class="status">' + jsonObj.statuses[current_status] + '</div>';
    }

    //little debug output below the stuff
    document.getElementById('server-output').innerHTML = document.getElementById('server-output').innerHTML + event.data + ' </br>';

};

// When the connection opens, send a message
socket.onopen = function() {
    const onopen_response = {
        value: 'ready'
    }

    let req_as_json = JSON.stringify(onopen_response);

    socket.send(req_as_json);

    //socket.send('Hello from the client!');
};

// Handle connection errors
socket.onerror = function(error) {
    console.error('WebSocket error:', error);
};

// Handle connection closure
socket.onclose = function() {
    console.log('WebSocket connection closed');
};