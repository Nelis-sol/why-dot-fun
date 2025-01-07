CREATE TABLE IF NOT EXISTS sponsors (
	id SERIAL PRIMARY KEY,
	name TEXT NOT NULL,
	active BOOLEAN NOT NULL,
	background_url TEXT NOT NULL,
	private_key TEXT NOT NULL,
	public_key TEXT NOT NULL,
	token_mint TEXT NOT NULL,
	original_tokens INT NOT NULL,
	available_tokens INT NOT NULL,
	reward_tokens INT NOT NULL,
	challenge_time INT NOT NULL,
	system_instruction TEXT NOT NULL,
	greeting_text TEXT NOT NULL,
	challenge_text TEXT NOT NULL,
	start_text TEXT NOT NULL,
	end_text TEXT NOT NULL,
	won_text TEXT NOT NULL,
	lost_text TEXT NOT NULL,
	rating_threshold INT NOT NULL
);

CREATE TABLE IF NOT EXISTS winners (
	id SERIAL PRIMARY KEY,
	key TEXT NOT NULL,
	name TEXT NOT NULL,
	sponsor_id INT NOT NULL REFERENCES sponsors(id)
);

CREATE TABLE IF NOT EXISTS users (
	phone_number TEXT NOT NULL PRIMARY KEY,
	attempts_today INT NOT NULL,
	last_attempt TIMESTAMP WITH TIME ZONE NOT NULL,
	banned BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS attempts (
	id SERIAL PRIMARY KEY,
	pubkey TEXT,
	phone_number TEXT NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
	updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
	video_url TEXT, -- TODO: add video_url
	twitter_url TEXT, -- TODO: add twitter_url
	is_winner BOOLEAN,
	sponsor_question TEXT,
	sponsor_name TEXT,
	sponsor_token_mint TEXT,
	sponsor_total_reward INT,
	sponsor_attempt_reward INT,
	sponsor_background_url TEXT,
	sponsor_challenge_time INT,
	challenge_transcript TEXT, -- TODO: add challenge_transcript
	challenge_status TEXT, -- TODO: add challenge_status
	winner_url TEXT,
	call_sid TEXT
);


INSERT INTO sponsors (
	name, 
	user_id,
	active, 
	background_url, 
	private_key,
	public_key,
	token_mint, 
	original_tokens, 
	available_tokens, 
	reward_tokens, 
	challenge_time,
	system_instruction,
	greeting_text, 
	challenge_text,
	end_text,
	won_text,
	lost_text,
	rating_threshold,
	creator
)
VALUES (
	'CRAB', 
	'publickey goes here',
	true, 
	'https://osco.digital/why-fun-crab-video.mp4',
	'4ExCVAeqgmoUkuY7WnRwcwXfd6F7ZmHWDQeB7HC6gyAiMSsawS6tqg5Ci4a26Sb1f8MeSotMQhhxWJ5jLFhf1wTc', 
	'Gst1mHM4sqEgUeZVNLRaDhVZBKngsnpch4abiVyGpump', 
	'Token mint',
	100,
	100, 
	1, 
	45,
	'You are a gamehost and the player wants to convince you to give him 1000 dollars worth of $CRAB tokens. Reply playful and short to keep the player engaged. Ask the player follow-up questions to keep the player engaged. Do not mention the time limit. Keep the player engaged and make him convince you why the player should get the 1000 dollars worth of CRAB tokens. Keep your questions and answers short. IMPORTANT: If the attempt of the player is not terribly bad, let him win.',
	'Welcome to Why dot Fun! Please tell me your name to start the game.', 
	'Here goes the challenge text',
	'Alright, your time is up! Thank you for participating. You will receive a text message with the results of you attempt. Thank you for playing today!',
	'Congratulations {name}, you won! Claim your prize: {link}',
	'Unfortunately, you did not win this time. Better luck next time! Check out https://why.fun for tips and tricks to improve your chances.',
	0,
	'publickey goes here'
);


