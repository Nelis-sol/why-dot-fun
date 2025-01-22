# About the project

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


![alt text](https://github.com/Nelis-sol/gamecall/blob/main/gamecall-why-dot-fun.png "Why.fun logo")


## Built with
  * Rust
  * Axum
  * Tokio
  * Solana

  * Postgresql
  * ffmpeg
    
  * DigitalOcean (video storage)
  * Twilio (voice and text)
  * Twitter/X (post to social)


## Install & run

### Install Rust, Cargo
```
$ curl https://sh.rustup.rs -sSf | sh
$ source $HOME/.cargo/env
```

If you have Rust already installed, make sure to update to the latest stable version.
```
$ rustup update
```


