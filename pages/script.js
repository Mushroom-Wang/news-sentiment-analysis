const form = document.getElementById('search-form');
form.addEventListener('submit', async (event) => {
    event.preventDefault();

    const searchBar = document.getElementById('search-bar');
    const searchTerm = searchBar.value;

    const loader = document.getElementById("loader");
    loader.style.display = 'block'; // Show the loader

    const response = await fetch(`/search?term=${encodeURIComponent(searchTerm)}`, {
        method: 'GET',
    });

    if (response.ok) {
        const results = await response.json();
        display(results);
    } else {
        // handle errors
    }
});

function formatDate(dateString) {
    const date = new Date(dateString);
    return date.toLocaleString();
}

function formatConfidence(confidence) {
    return (confidence * 100).toFixed(2) + '%';
}

function truncateText(text, wordLimit) {
    const words = text.split(' ');
    if (words.length > wordLimit) {
        return words.slice(0, wordLimit).join(' ') + '...';
    }
    return text;
}

function display(data) {
    const container = document.getElementById("jsonDataContainer");
    container.innerHTML = '';
    data.forEach(item => {
        const textBox = document.createElement("div");
        textBox.className = "textbox";
        const sentimentColor = item.sentiment === "NEGATIVE" ? "red" : "green";
        const truncatedTitle = truncateText(item.title, 20);
        textBox.innerHTML = `
            <p>Sentiment: <span class="sentiment-dot" style="background-color: ${sentimentColor};"></span>${item.sentiment}</p>
            <p>Confidence: ${formatConfidence(item.confidence)}</p>
            ${item.image ? `<img src="${item.image}" alt="${truncatedTitle}" width="100%">` : `<img src="https://img.freepik.com/premium-photo/portrait-adorable-lazy-cat-sunglasses-laying-hammock-ocean-beach-enjoying_245726-3884.jpg" alt="${truncatedTitle}" width="100%">`}
            <h3 href="${item.url}" target="_blank">${truncatedTitle}</h3>
            <p>Date: ${formatDate(item.date)}</p>
            <p>Source: ${item.source}</p>
            <a href="${item.url}" target="_blank">Read More</a>
        `;
        //const truncatedBody = truncateText(item.body, 40); <p>Body: ${truncatedBody}</p>
        container.appendChild(textBox);
    });
    const loader = document.getElementById("loader");
    loader.style.display = 'none'; // Hide the loader
}