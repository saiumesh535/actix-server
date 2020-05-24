// Example POST method implementation:
async function postData(url = '', data = {}) {
    // Default options are marked with *
    const response = await fetch(url, {
        method: 'POST', // *GET, POST, PUT, DELETE, etc.
        mode: 'cors', // no-cors, *cors, same-origin
        cache: 'no-cache', // *default, no-cache, reload, force-cache, only-if-cached
        credentials: 'same-origin', // include, *same-origin, omit
        withCredentials: true,
        headers: {
            'Content-Type': 'application/json'
            // 'Content-Type': 'application/x-www-form-urlencoded',
        },
        redirect: 'follow', // manual, *follow, error
        referrerPolicy: 'no-referrer', // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
        body: JSON.stringify(data) // body data type must match "Content-Type" header
    });
    return response; // parses JSON response into native JavaScript objects
}


(() => {
    setTimeout(() => {
        document.querySelector("#loginButton").onclick = async () => {
            const email = document.querySelector('.container > input:nth-child(1)').value;
            const password = document.querySelector('.container > input:nth-child(2)').value;
            try {
                const response = await postData('http://127.0.0.1:8000/auth/login', {
                    email, password
                });
                if(response.status !== 200) {
                    alert('check email and password');
                    return;
                }
                const responseBody = await response.json();
                alert(responseBody.token);
            } catch(error) {
                console.error(error);
            }
        }
    })
})();