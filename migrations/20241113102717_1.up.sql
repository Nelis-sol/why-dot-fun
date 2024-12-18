CREATE TABLE IF NOT EXISTS sponsors (
	id SERIAL PRIMARY KEY,
	name TEXT NOT NULL,
	active BOOLEAN NOT NULL,
	background_url TEXT NOT NULL,
	private_key TEXT NOT NULL,
	token_mint TEXT NOT NULL,
	original_tokens INT NOT NULL,
	available_tokens INT NOT NULL,
	reward_tokens INT NOT NULL,
	challenge_time INT NOT NULL,
	system_instruction TEXT NOT NULL,
	greeting_text TEXT NOT NULL,
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
	phone_number TEXT NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL,
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
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
	challenge_status TEXT -- TODO: add challenge_status
);


INSERT INTO sponsors (
	name, 
	active, 
	background_url, 
	private_key,
	token_mint, 
	original_tokens, 
	available_tokens, 
	reward_tokens, 
	challenge_time,
	system_instruction,
	greeting_text, 
	start_text, 
	end_text,
	won_text,
	lost_text,
	rating_threshold
)
VALUES (
	'Airbnb', 
	true, 
	'https://images.surferseo.art/82b37c69-0150-4d5e-b5b4-4a2a87b31a21.jpeg',
	'Private key', 
	'Token mint', 
	1000, 
	1000, 
	100, 
	20,
	'The player wants to convince you to give them a free stay at any of your apartments. Reply playful and short to keep the player engaged. Do not mention the time limit',
	'Welcome to the Airbnb apartment giveaway! Please tell us your name to start the game.', 
	'Thank you {name}. Let''s start the game. You will have {duration} seconds to convince us to give you a free stay at any of our apartments! Go!', 
	'Your time is up! Thank you for participating. We will now decide who gets the free apartment and send you a message. Goodbye!',
	'Congratulations {name}! You won the free stay at any of our apartments! Visit {link} to claim your prize.',
	'Hey {name}, unfortunately, you lost the game. Better luck next time!',
	10
);


