const express = require('express');
const path = require('path');
const app = express();
const port = 3000;

app.use('/css', express.static('css'))
app.use('/img', express.static('img'))
app.get('/', (req, res) =>
	res.sendFile(path.join(__dirname + '/index.html'))
);

app.get('/demo', (req, res) => {
	res.set('X-demoing', 'just playing arounf');
	res.status(418);
	res.send('So now... what?');
});

app.listen(port, () => console.log(`Example app listening in port ${port}!`));
