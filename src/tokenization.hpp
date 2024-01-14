#pragma once

#include <iostream>
#include <string>
#include <vector>
#include <optional>

enum class TokenType {
    exit,
    int_lit,
    semicolon,
    rparen,
    lparen,
    identifier,
};

struct Token {
    TokenType type;
    std::optional<std::string> value{};
};

class Tokenizer {
public:
    inline explicit Tokenizer(const std::string& src)
        : m_src(std::move(src)) {}

    inline std::vector<Token> tokenize() {
        std::vector<Token> tokens;
        std::string buffer{};
        while (peek().has_value()) {
            if (std::isalpha(peek().value())) {
                buffer.push_back(consume());
                while (peek().has_value() && std::isalnum(peek().value())) {
                    buffer.push_back(consume());
                }
                if (buffer == "exit") {
                    tokens.push_back(Token{TokenType::exit});
                    buffer.clear();
                    continue;
                } else {
                    tokens.push_back(Token{TokenType::identifier, buffer});
                    buffer.clear();
                    continue;
                }
            } else if (std::isdigit(peek().value())) {
                buffer.push_back(consume());
                while (peek().has_value() && std::isdigit(peek().value())) {
                    buffer.push_back(consume());
                }
                tokens.push_back({TokenType::int_lit, buffer});
                buffer.clear();
                continue; 
            } else if (peek().value() == '(') {
                consume();
                tokens.push_back({TokenType::lparen});
                continue;
            } else if (peek().value() == ')') {
                consume();
                tokens.push_back({TokenType::rparen});
                continue;
            } else if (peek().value() == ';') {
                tokens.push_back({TokenType::semicolon});
                buffer.clear();
                consume();
                continue;
            } else if (std::isspace(peek().value())) {
                consume();
                continue;
            } else {
                exit(EXIT_FAILURE);
            }
        }
        m_index = 0;
        return tokens;
    }

private:

    [[nodiscard]] inline std::optional<char> peek(int offset = 0) const {
        if (m_index + offset >= m_src.length()) {
            return {};
        } else {
            return m_src.at(m_index + offset);
        }
    }

    inline char consume(){
        return m_src.at(m_index++);
    }

    const std::string m_src;
    size_t m_index = 0;
};