import React from 'react';
import './ChatMessage.css';

function ChatMessage({ message }) {
  const isBot = message.sender === 'bot';
  const isError = message.isError || false;

  return (
    <div className={`chat-message ${isBot ? 'bot' : 'user'} ${isError ? 'error' : ''}`}>
      <div className="message-avatar">
        {isBot ? '🤖' : '👤'}
      </div>
      <div className="message-content">
        <div className="message-text">{message.text}</div>
        {message.confidence && (
          <div className="message-confidence">
            Confiança: {(message.confidence * 100).toFixed(0)}%
          </div>
        )}
        <div className="message-time">
          {message.timestamp.toLocaleTimeString('pt-BR')}
        </div>
      </div>
    </div>
  );
}

export default ChatMessage;
