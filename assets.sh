#!/bin/bash

# Définition des fichiers Google Drive (ID et noms)
declare -A files=(
    ["1h1E6wJHrlwLTN0W0J3ELUwvqKwGDSll5"]="enemy.glb"
    ["129wjq420NXlSONoYdB3wU1_8clcpXULw"]="player.glb"
)

ASSETS_DIR="assets"
mkdir -p "$ASSETS_DIR"

# Fonction pour télécharger un fichier depuis Google Drive
download_file() {
    FILE_ID=$1
    FILE_NAME=$2

    echo "Téléchargement de ${FILE_NAME}..."

    # Obtenir le lien de confirmation et télécharger le fichier temporaire
    wget --no-check-certificate "https://drive.google.com/uc?export=download&id=${FILE_ID}" -O temp.html

    # Extraire le code de confirmation (si nécessaire)
    CONFIRM_CODE=$(cat temp.html | grep -o 'confirm=[^&]*' | sed 's/confirm=//')

    # Télécharger le fichier avec confirmation dans le dossier assets
    wget --no-check-certificate "https://drive.google.com/uc?export=download&confirm=${CONFIRM_CODE}&id=${FILE_ID}" -O "${ASSETS_DIR}/${FILE_NAME}"

    # Nettoyage des fichiers temporaires
    rm temp.html

    echo "Fichier téléchargé : ${ASSETS_DIR}/${FILE_NAME}"
}

# Boucle sur chaque fichier à télécharger
for FILE_ID in "${!files[@]}"; do
    download_file "$FILE_ID" "${files[$FILE_ID]}"
done

echo "Tous les fichiers ont été téléchargés et placés dans le dossier '${ASSETS_DIR}/'."
