<script>
  import { invoke } from '@tauri-apps/api/core';
  import { afterUpdate } from 'svelte';

  export let messages = [];
  export let inputText = '';
  export let isLoading = false;

  let textarea;
  let messageList;

  async function sendMessage() {
    const trimmed = inputText.trim();
    if (!trimmed || isLoading) return;
    messages = [...messages, { role: 'user', content: trimmed }];
    inputText = '';
    isLoading = true;
    adjustHeight();
    try {
      const reply = await invoke('send_message', { prompt: trimmed });
      messages = [...messages, { role: 'assistant', content: reply }];
    } catch (e) {
      messages = [...messages, { role: 'assistant', content: `Fel: ${e}` }];
    } finally {
      isLoading = false;
    }
  }

  function handleKeydown(e) {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      sendMessage();
    }
  }

  function adjustHeight() {
    if (!textarea) return;
    textarea.style.height = 'auto';
    const maxRows = 6;
    const lineHeight = parseInt(getComputedStyle(textarea).lineHeight);
    const maxHeight = lineHeight * maxRows;
    textarea.style.height = Math.min(textarea.scrollHeight, maxHeight) + 'px';
  }

  afterUpdate(() => {
    if (messageList) {
      messageList.scrollTop = messageList.scrollHeight;
    }
  });
</script>

<div class="chat-view">
  <div class="messages" bind:this={messageList}>
    {#each messages as { role, content }}
      <div class="message {role}">
        <div class="bubble">{content}</div>
      </div>
    {/each}
    {#if isLoading}
      <div class="loading">
        <div class="dot"></div>
      </div>
    {/if}
  </div>

  <div class="input-area">
    <textarea
      bind:this={textarea}
      bind:value={inputText}
      rows="1"
      placeholder="Write a message..."
      on:input={adjustHeight}
      on:keydown={handleKeydown}
    ></textarea>
    <button class="send" on:click={sendMessage}>Send</button>
  </div>
</div>

<style>
  :root {
    --accent: #CC785C;
    --bg-dark: #1a1a1a;
    --text-dark: #E8E3DD;
    --assistant-bg: #F5F0EB;
    --assistant-text: #1a1a1a;
  }

  .chat-view {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--bg-dark);
    color: var(--text-dark);
  }

  .messages {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .message {
    display: flex;
  }
  .message.user {
    justify-content: flex-end;
  }
  .message.assistant {
    justify-content: flex-start;
  }

  .bubble {
    max-width: 70%;
    padding: 0.6rem 0.9rem;
    border-radius: 0.8rem;
    line-height: 1.4;
    white-space: pre-wrap;
  }
  .message.user .bubble {
    background: var(--accent);
    color: #fff;
  }
  .message.assistant .bubble {
    background: var(--assistant-bg);
    color: var(--assistant-text);
  }

  .loading {
    display: flex;
    justify-content: flex-start;
    padding-left: 0.2rem;
  }
  .dot {
    width: 10px;
    height: 10px;
    background: var(--accent);
    border-radius: 50%;
    animation: pulse 1.5s infinite;
  }
  @keyframes pulse {
    0%, 100% { transform: scale(1); }
    50% { transform: scale(1.4); }
  }

  .input-area {
    display: flex;
    gap: 0.5rem;
    padding: 0.5rem;
    border-top: 1px solid #444;
  }
  textarea {
    flex: 1;
    resize: none;
    overflow: hidden;
    border: none;
    border-radius: 0.5rem;
    padding: 0.5rem;
    font-family: inherit;
    font-size: 1rem;
    background: #fff;
    color: #000;
  }
  textarea:focus {
    outline: none;
  }
  button.send {
    background: var(--accent);
    color: #fff;
    border: none;
    border-radius: 0.5rem;
    padding: 0 1rem;
    cursor: pointer;
  }
  button.send:hover {
    opacity: 0.9;
  }

  .footer {
    text-align: center;
    padding: 0.5rem;
    font-size: 0.85rem;
    color: var(--text-dark);
  }
</style>