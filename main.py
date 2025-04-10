import os
import sys
from openai import OpenAI
from dotenv import load_dotenv
from rich.console import Console
from rich.markdown import Markdown

load_dotenv()

api_key = os.getenv("OPENAI_API_KEY")
client = OpenAI(api_key=api_key)
console = Console()

def chat(messages):
    completion = client.chat.completions.create(
        model="gpt-4.5-preview",
        messages=messages,
        stream=True
    )

    return completion

def print_stream(completion, messages):
    response = []
    for chunk in completion:
        if type(chunk.choices[0].delta.content) is str:
            sys.stdout.write(chunk.choices[0].delta.content)
            sys.stdout.flush()
            response.append(chunk.choices[0].delta.content)
        else: 
            pass

    # Combine the streamed response into a single string
    return "".join(response)

def display_conversation(messages):
    # Output markdown-rendered responses
    os.system("clear")
    for message in messages:
        string = Markdown(message['content'])
        if message['role'] == 'user':
            console.print(f"[bold green]{message['role']}")
        elif message['role'] == 'assistant':
            console.print(f"[bold blue]{message['role']}")
        console.print(string)
        console.print()

def main():
    messages = [{"role": "system", "content": "You are a chat assistant living in my computer terminal. I am probably trying to get quick answers."}]
    while True:
        user_input = input("> ")

        if user_input in ["exit", ":q"]:
            return None

        user_message = {"role": "user", "content": user_input}
        messages.append(user_message)
        response = chat(messages)
        messages.append({"role": "assistant", "content": print_stream(response, messages)})
        display_conversation(messages)
        print('\n')

if __name__ == "__main__":
    main()
