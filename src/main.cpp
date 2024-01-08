#include <fstream>
#include <iostream>
#include <optional>
#include <sstream>
#include <vector>

#include "tokenization.hpp"
#include "parser.hpp"
#include "generation.hpp"

int main(int argc, char* argv[]) {
    if (argc != 2) {
        std::cerr << "Incorrect usage. Correct usage is :" << std::endl;
        std::cerr << "ori <input.ori>" << std::endl;
        return EXIT_FAILURE;
    }

    std::string contents;
    {  // scope to close the file by itself
        std::fstream input(argv[1], std::ios::in);
        std::stringstream contents_stream;
        contents_stream << input.rdbuf();
        contents = contents_stream.str();
    }
    Tokenizer tokenizer(std::move(contents));
    std::vector<Token> tokens = tokenizer.tokenize();

    Parser parser(std::move(tokens));
    std::optional<NodeExit> tree = parser.parse();
    if (!tree.has_value()) {
        std::cerr << "No exit statement found" << std::endl;
        exit(EXIT_FAILURE);
    }

    Generator generator(tree.value());

    {
        std::fstream file("F:/programmation/c/C++/projets/ori_compiler/out/out.asm", std::ios::out);
        
        file << generator.generate();
    }

    system("powershell -ExecutionPolicy ByPass -File F:\\programmation\\c\\C++\\projets\\ori_compiler\\asm_to_exe.ps1");

    return EXIT_SUCCESS;
}