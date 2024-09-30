const socket = new WebSocket('ws://192.168.1.169:3032/ws');

console.log('hello from js!');
// Listen for messages from the server
socket.onmessage = function(event) {
    console.log('Message from server:', event.data);
    document.getElementById('server-output').innerHTML = document.getElementById('server-output').innerHTML + 'Message from server: ' + event.data + ' </br>'
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