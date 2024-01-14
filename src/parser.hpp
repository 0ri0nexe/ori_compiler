#pragma once

#include <vector>
#include <variant>

#include "tokenization.hpp"

struct NodeExprIntLitteral {
    Token int_lit;
};

struct NodeExprIdentifier {
    Token identifier;
};

struct NodeExpr {
    std::variant<NodeExprIntLitteral, NodeExprIdentifier> var;
};

struct NodeStatementExit {
    NodeExpr expr;
};

struct NodeStatementLet {
    Token indent;
    NodeExpr expr;
};

struct NodeStatement {
    std::variant<NodeStatementExit, NodeStatementLet> var;
};

struct NodeProgram {
    std::vector<NodeStatement> var;
};
struct NodeExit {
    NodeExpr expr;
};


class Parser {
public:
    inline explicit Parser(std::vector<Token> tokens)
    : m_tokens(std::move(tokens))
    {}

    inline std::optional<NodeExpr> parse_expr() {
        if (peek().has_value() && peek().value().type == TokenType::int_lit) { 
            return NodeExpr{NodeExprIntLitteral{consume()}};
        } else if (peek().has_value() && peek().value().type == TokenType::identifier){
            return NodeExpr {NodeExprIdentifier{consume()}};
        } else {
            return {};
        }
    }

    inline std::optional<NodeExit> parse() {
        std::optional<NodeExit> exit_node;
        while (peek().has_value()) {
            if (peek().value().type == TokenType::exit && peek(1).has_value() && peek(1).value().type == TokenType::lparen) {
                consume();
                consume();
                // the line below mean that if the method returns a value then node_expre will take true
                // and the code will be executed, if not the else statement will be executed
                if (auto node_expr = parse_expr()) {
                    exit_node = NodeExit{node_expr.value()};
                } else {
                    std::cerr << "Invalid expression" << std::endl;
                    exit(EXIT_FAILURE);
                }
                if (peek().has_value() && peek().value().type == TokenType::rparen) {
                    consume();
                } else {
                    std::cerr << "Expected `)`" << std::endl;
                }
                if (peek().has_value() && peek().value().type == TokenType::semicolon) {
                    consume();
                } else {
                    std::cerr << "Expected `;`" << std::endl;
                    exit(EXIT_FAILURE);
                }
            }
        } 
        m_index = 0;
        return exit_node;
    }

private:

    [[nodiscard]] inline std::optional<Token> peek(int offset = 0) const {
        if (m_index + offset >= m_tokens.size()) {
            return {};
        } else {
            return m_tokens.at(m_index + offset);
        }
    }

    inline Token consume(){
        return m_tokens.at(m_index++);
    }

    const std::vector<Token> m_tokens;
    size_t m_index = 0;
};