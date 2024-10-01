const socket = new WebSocket('ws://192.168.1.169:3032/ws');

console.log('hello from js!');
// Listen for messages from the server
socket.onmessage = function(event) {
    console.log('Message from server:', event.data);

    //messages back from the server are going to start coming in here as deserialized json
    //the first message back will be the entire board as json
    //since the board comes back in this way here is where the bulk of the web page needs to be generated

    //here is where the logic will be that takes the response types into consideration, and operates on the DOM to show visual changes

    //logic needs to be written here (similar to the cli that interprets individual changes incoming from the server, and update the respresentation on the DOM)
    //small changes would be ideal and updated live because all the potentially connected clients are going to receive these at the same time when their updates live!

    document.getElementById('server-output').innerHTML = document.getElementById('server-output').innerHTML + event.data + ' </br>'
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