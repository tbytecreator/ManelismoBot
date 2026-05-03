import React, { useState } from 'react';
import './ChatInput.css';

function ChatInput({ onSendMessage, loading }) {
  const [input, setInput] = useState('');

  const handleSubmit = (e) => {
    e.preventDefault();
    if (input.trim() && !loading) {
      onSendMessage(input);
      setInput('');
    }
  };

  const handleKeyPress = (e) => {
    if (e.key === 'Enter' && !e.shiftKey && !loading) {
      e.preventDefault();
      handleSubmit(e);
    }
  };

  return (
    <form className="chat-input-form" onSubmit={handleSubmit}>
      <div className="input-container">
        <textarea
          className="chat-input"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyPress={handleKeyPress}
          placeholder="Digite sua pergunta aqui... (Shift+Enter para quebra de linha)"
          disabled={loading}
          rows="2"
        />
        <button
          type="submit"
          className="send-button"
          disabled={!input.trim() || loading}
        >
          {loading ? '...' : '➤'}
        </button>
      </div>
    </form>
  );
}

export default ChatInput;
