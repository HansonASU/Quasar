# Quasar: OpenAI Chatbot
Quasar is a command-line chatbot that uses OpenAI’s GPT-3.5-turbo model to generate responses. This app allows users to have a conversation with the AI model by simply typing their messages in the terminal.

### Prerequisites
* Rust: Install the Rust programming language  [here](https://www.rust-lang.org/tools/install) .
* OpenAI API Key: Sign up for an OpenAI account  [here](https://beta.openai.com/signup/)  to get an API key. Note that the GPT-3.5-turbo model may require a subscription to use.

### Setup
1. Clone the repository:
Copy code:
```
git clone https://github.com/HansonASU/Quasar.git 
cd Quasar 
```

2. Set up an environment variable with your OpenAI API key:

* For Linux/macOS:
Copy code:
```
export OPENAI_API_KEY=your_api_key_here 
```

* For Windows (Command Prompt):
Copy code:
```
set OPENAI_API_KEY=your_api_key_here 
```

* For Windows (PowerShell):
Copy code
```
$env:OPENAI_API_KEY=“your_api_key_here” 
````

3. Compile and run the project:
Copy code:
```
cargo build —release cargo run —release 
```

The chatbot should now be running, and you can start chatting with it.

### Usage
Once the application is running, you will see a prompt that says “You: “. Type your message and press Enter to send it. The chatbot will generate a response, which will be displayed as “Quasar: [response]”.
To exit the application, press Ctrl + C.

Note: There is a line in main.rs, `max_tokens:` that is set to 3000. This is the max tokens the API will use for the reply. Feel free to set this as you like.

### Contributing
If you find any issues, please feel free to create an issue on GitHub. Pull requests are also welcome!

### License
This project is licensed under the MIT License - see the  [LICENSE](https://chat.openai.com/LICENSE)  file for details.


