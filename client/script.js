document.addEventListener("DOMContentLoaded", () => {
    const resultParagraph = document.getElementById("result-paragraph");

    
    function fetchData(endpoint) {
        fetch(`http://localhost:8000${endpoint}`)
            .then(res => {
                if (!res.ok) {
                    throw new Error(`HTTP error! Status: ${res.status}`);
                }
                return res.text(); 
            })
            .then(data => {
                console.log(data);
                resultParagraph.innerHTML = data; 
            })
            .catch(err => {
                console.error(err);
                resultParagraph.innerHTML = "Error occurred";
            });
    }

    document.getElementById("button-1").addEventListener("click", () => fetchData("/name"));
    document.getElementById("button-2").addEventListener("click", () => fetchData("/branch"));
    document.getElementById("button-3").addEventListener("click", () => fetchData("/college"));
    document.getElementById("button-4").addEventListener("click", () => fetchData("/department"));
    document.getElementById("button-5").addEventListener("click", () => fetchData("/involvements"));
});

document.addEventListener("DOMContentLoaded", function() {
    fetch('http://127.0.0.1:8000/visitor_count')
        .then(response => response.text())
        .then(data => {
            document.getElementById('visitor-count').innerText = `Visitor Count: ${data}`;
        })
        .catch(error => {
            console.error('Error fetching visitor count:', error);
        });
});
