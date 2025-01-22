# About the project

![alt text](https://github.com/Nelis-sol/gamecall/blob/main/gamecall-why-dot-fun.png "Why.fun logo")
<br />

## Gamecall (otherwise known as why.fun)

With gamecall, users can call with an AI agent and complete a challenge. The AI agent has access to a **phonenumber** (through Twilio), a **crypto-wallet** (on Solana), a **social media account** (on Twitter/X). 
Additionally, the AI agent has capabailities to **generate video content** based on the conversation it has with the caller. Anyone can submit a new challenge through the API (a 1 SOL fee + prizepool for winning callers). This includes prompting the AI agent and instructing it on how to judge winning and losing attempts. Gamecall also allows calls through the browser.

**The AI agent works fully autonomously**:
  * interacting and having conversations with the caller
  * judging the attempt, (3) paying out the prize
  * texting the result to the caller
  * generating video content
  * moderate/decide if content is suitable for posting on socials
  * write the tweet and post it on X witht the generated video of the call interaction

<br /><br />

## Built with
  - Rust
  - Axum  
  - Tokio  
  - Solana
<br />
  - Postgresql
  - ffmpeg
<br />
  - DigitalOcean (video storage)
  - Twilio (voice and text)
  - Twitter/X (post to social)

<br />
____
<br />

## Install & run
### 1. Install Rust, Cargo
```
$ curl https://sh.rustup.rs -sSf | sh
$ source $HOME/.cargo/env
```

If you have Rust already installed, make sure to update to the latest stable version.
```
$ rustup update
```
<br /><br />

### 2. Set up environment variables
Some environment variables are required to be present during the build phase (e.g. the database url for sqlx). So setting up the environment variables before continuing the next steps is recommended. The database url is required to set up before building, the others are optional. 

<br />

### 3. Set up tables in postgresql database

   #### 3.1 Option 1: Use sqlx migrations
   Run sqlx migration to set up tables in the database. 
   ```
   sqlx migrate run
   ```

   #### 3.2 Option 2: Set up tables manually
   Set up tables in the database manually. 
   Use the queries in this file: 
   ![alt_text](https://github.com/Nelis-sol/gamecall/blob/main/migrations/20241113102717_1.up.sql "Queries to set up tables")


### Run program

   #### Option 1: Build and run directly

   ```
   DATABASE_URL=<your-database-connection-string> cargo build --release
   cargo run --release 
   ```
   <br />

   #### Option 2: Automatically build and run with Digital Ocean

   1. Login to Digital Ocean
   2. App Platform
   3. Create app
   4. Authorize Digital Ocean to access repositories
   5. Select your (gamecall) repository 
   6. Select auto-deploy (if not selected already)
   7. Set environment variables
   8. Complete rest of the set up with default settings

   Now everytime you push a commit to your repository, it will be build and deployed automatically on DigitalOcean.
   <br />

   #### Option 3: Build and run using Docker

   ```
   docker build -t webcall .
   docker run -p 8080:8080 -t webcall
   ```







