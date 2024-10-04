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
    <div id = "edit-content-modal">New content: <input type = "text" name="edit-content-input-box" id="edit-content-input">
    <button type="button" id="edit-button">Edit!</button></div>
    `;

function build_board(response) {
    
    let parser = new DOMParser();
    const doc = parser.parseFromString(htmlString, 'text/html');
    doc.getElementById('board-name').innerHTML = response.board_name;

    for(let current_status = 0; current_status < response.statuses.length; current_status++) {

        let status = response.statuses[current_status];

        let col_by_id = doc.getElementById('status-columns');

        col_by_id.innerHTML += '<div id="status-' + status + '">'+status.toUpperCase()+'<div id = "item-box-'+status+'"></div>';

        for (let current_item = 0; current_item < response.items.length; current_item++) {

            let item = response.items[current_item];

            if (item.status == status) {

                let item_box_by_id = doc.getElementById('item-box-'+status);
                item_box_by_id.innerHTML +=
                    '<div id="item-cont">'+
                        '<div id="item-name">'+
                            '<div id ="name-container">'+ item.name + '</div>'+
                            '<div id ="demote-box" class="'+item.name+'-demote-clicky">↓</div>'+
                            '<div id ="promote-box" class="'+item.name+'-promote-clicky">↑</div>'+
                            '<div id ="edit-content-box" class="'+item.name+'-edit-clicky">✎</div>'+
                            '<div id ="remove-item-box" class="'+item.name+'-remove-clicky">✖</div>'+
                        '</div>'+
                        '<div id="item-contents">'+item.contents + '</div></div>'+
                    '</div>';
            }
        }
    }

    doc.getElementById('server-output').innerHTML += event.data + ' </br>';

    document.getElementsByTagName('body')[0].innerHTML = doc.documentElement.outerHTML;
    
}

function show_edit_modal(edit_content_box) {
    let modal = document.getElementById('edit-content-modal');
    modal.className = edit_content_box.slice(0, -12);
    console.log('set modal class to: ' + modal.className);
    modal.style.display = 'block';
    modal_is_shown = true;
}

function add_new_item() {

    let input_name = document.getElementById('item-name-input').value;
    let input_content = document.getElementById('item-content-input').value;

    if (input_name != '' && input_content != '') {
        console.log('sending add request...');
        const response = {
            command: 'add',
            args: [input_name, input_content],
        }
        socket.send(JSON.stringify(response));
    } else {
        console.log('field was left empty!');
    }
}

function edit_from_modal() {
    let input_box = document.getElementById('edit-content-input').value;
    let item_selected = document.getElementById('edit-content-modal').className;
    if (input_box != '') {
        console.log('sending edit request...');
        const response = {
            command: 'edit_content',
            args: [item_selected, input_box],
        }
        socket.send(JSON.stringify(response));
    }
}

function demote_func(demote_box) {
    console.log('sending demote request...');
    let demote_box_item = demote_box.slice(0, -14);
    const response = {
        command: 'demote',
        args: [demote_box_item],
    }
    socket.send(JSON.stringify(response));
}

function promote_func(promote_box) {
    console.log('sending promote request...');
    let promote_box_item = promote_box.slice(0, -15);
    const response = {
        command: 'promote',
        args: [promote_box_item],
    }
    socket.send(JSON.stringify(response));
}

function remove_item_func(remove_item_box) {
    console.log('sending remove item request...');
    let remove_item_box_item = remove_item_box.slice(0, -14);
    const response = {
        command: 'remove',
        args: [remove_item_box_item],
    }
    socket.send(JSON.stringify(response));
}

console.log('starting web socket...');

const socket = new WebSocket('ws://192.168.1.169:3032/ws');

socket.onmessage = function(event) {
    if (event.data != '') {
        build_board(JSON.parse(event.data));

        document.getElementById('add-button').addEventListener("click", add_new_item);
        document.getElementById('edit-button').addEventListener("click", edit_from_modal);
    
        const demote_boxes = document.querySelectorAll('#demote-box');
        demote_boxes.forEach(box => {
            document.getElementsByClassName(box.className)[0].addEventListener("click", () => demote_func(box.className));
        });

        const promote_boxes = document.querySelectorAll('#promote-box');
        promote_boxes.forEach(box => document.getElementsByClassName(box.className)[0].addEventListener("click", () => promote_func(box.className)));

        const edit_content_boxes = document.querySelectorAll('#edit-content-box');
        edit_content_boxes.forEach(box => document.getElementsByClassName(box.className)[0].addEventListener("click", () => show_edit_modal(box.className)));

        const remove_item_boxes = document.querySelectorAll('#remove-item-box');
        remove_item_boxes.forEach(box => document.getElementsByClassName(box.className)[0].addEventListener("click", () => remove_item_func(box.className)));

        const modal_box = document.getElementById('edit-content-modal');

        document.addEventListener('click', (event) => {
            if(!modal_box.contains(event.target)) {
                modal_box.style.display = 'none';
            }
        }, true);

    } else {
        console.log("server ignored sent request");
    }
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