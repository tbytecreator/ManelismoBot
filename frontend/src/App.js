import React, { useState, useRef, useEffect } from 'react';
import './App.css';
import ChatMessage from './components/ChatMessage';
import ChatInput from './components/ChatInput';

const API_BASE_URL = process.env.REACT_APP_API_URL || 'http://localhost:8080';
const BANNER_URL = process.env.REACT_APP_BANNER_URL || '/banner-tbytecreator.png';

function App() {
  const [messages, setMessages] = useState([
    {
      id: 1,
      text: "Chega mais. Manda a pergunta sem rodeio e eu respondo no estilo Manoel Neto: direto, crítico e sem verniz corporativo.",
      sender: 'bot',
      timestamp: new Date(),
    },
  ]);
  const [loading, setLoading] = useState(false);
  const messagesEndRef = useRef(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const handleSendMessage = async (question) => {
    const userMessage = {
      id: messages.length + 1,
      text: question,
      sender: 'user',
      timestamp: new Date(),
    };

    setMessages((prev) => [...prev, userMessage]);
    setLoading(true);

    try {
      const response = await fetch(`${API_BASE_URL}/api/ask`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ question }),
      });

      if (!response.ok) {
        throw new Error('Network response was not ok');
      }

      const data = await response.json();
      const botMessage = {
        id: messages.length + 2,
        text: data.answer,
        sender: 'bot',
        timestamp: new Date(),
        confidence: data.confidence,
      };

      setMessages((prev) => [...prev, botMessage]);
    } catch (error) {
      console.error('Error:', error);
      const errorMessage = {
        id: messages.length + 2,
        text: 'Desculpe, ocorreu um erro ao processar sua pergunta. Tente novamente.',
        sender: 'bot',
        timestamp: new Date(),
        isError: true,
      };
      setMessages((prev) => [...prev, errorMessage]);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="App">
      <div className="container">
        <header className="chat-header">
          <div className="banner-shell">
            <div
              className="banner-image"
              style={{ backgroundImage: `url(${BANNER_URL})` }}
              role="img"
              aria-label="Banner TByteCreator"
            />
            <div className="banner-overlay" />
            <div className="banner-content">
              <p className="eyebrow">TBYTECREATOR</p>
              <h1>Bot do Manelismo</h1>
              <p className="subtitle">Visão crua sobre tecnologia, sociedade e cultura</p>
            </div>
          </div>
        </header>

        <div className="chat-messages">
          {messages.map((message) => (
            <ChatMessage key={message.id} message={message} />
          ))}
          {loading && (
            <div className="loading-indicator">
              <span className="dot"></span>
              <span className="dot"></span>
              <span className="dot"></span>
            </div>
          )}
          <div ref={messagesEndRef} />
        </div>

        <ChatInput onSendMessage={handleSendMessage} loading={loading} />
      </div>
    </div>
  );
}

export default App;
