import fs from "node:fs";
import path from "node:path";
import { Cookie } from "tough-cookie";
import { Scraper } from "agent-twitter-client";

const username = process.argv[2];
const password = process.argv[3];
const tweet = process.argv[4];

if (!username || !password || !tweet) {
	throw new Error("invalid arguments");
}

const cookiesPath = path.join(
	import.meta.dirname,
	"storage",
	"saved-cookies.json"
);
let cookies = [];
let saved = {};

try {
	saved = JSON.parse(fs.readFileSync(cookiesPath, "utf8"));
	cookies = saved[username];
	cookies = cookies.map((cookie) => Cookie.fromJSON(cookie));
} catch {}

const scraper = new Scraper();

if (cookies.length) {
	await scraper.setCookies(cookies);
} else {
	await scraper.login(username, password);
	cookies = await scraper.getCookies();
	saved[username] = cookies;

	fs.writeFileSync(cookiesPath, JSON.stringify(saved));
}

if (!(await scraper.isLoggedIn())) {
	throw new Error("invalid login details");
}

await scraper.sendTweet(tweet);
