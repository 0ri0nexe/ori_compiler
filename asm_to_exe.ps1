$filePath = "F:\programmation\c\C++\projets\ori_compiler\out\out.exe"

# Vérifier si le fichier existe
if (Test-Path $filePath) {
    # Si le fichier existe, le supprimer
    Remove-Item $filePath -Force
    Write-Output "Le fichier a été supprimé."
} else {
    Write-Output "Le fichier n'existe pas."
}

cd F:\programmation\c\C++\projets\ori_compiler\out\
nasm.exe -f win64 out.asm -o out.obj
golink.exe out.obj /entry main /console kernel32.dll