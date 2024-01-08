#pragma once

#include <sstream>

#include "parser.hpp"

class Generator {
public:
    inline Generator(NodeExit root) 
    : m_root(std::move(root))
    {}

    [[nodiscard]] inline std::string generate() const {
        std::stringstream output;
        output << "bits 64\n";
        output << "extern ExitProcess\n";
        output << "section .text\n";
        output << "\tglobal main\n";
        output << "\tmain:\n";
        std::string exitCode = m_root.expr.int_lit.value.value();
        output << "\t\tmov rcx, " << exitCode << "\n";
        output << "\t\tcall ExitProcess\n";
        return output.str();
    }
private:
    const NodeExit m_root; 
};