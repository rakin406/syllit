function isVowel(letter) {
  let vowels = "aeiou";
  for (let i = 0; i < vowels.length; ++i) {
    // Check if vowel is same as letter
    if (letter.toLowerCase() == vowels[i]) {
      return true;
    }
  }
  return false;
}

function showResult(data) {
  let textArea = document.getElementById("result");
  if (textArea.style.display === "none") {
    textArea.style.display = "block";
  }
  textArea.value = data;
}

function countSyllables() {
  let word = document.getElementById("word").value.toLowerCase();
  let prevChar = "";
  let count = 0;

  // Find syllables in word
  for (let i = 0; i < word.length; ++i) {
    // Check if letter is vowel
    if (isVowel(word[i]) || word[i] == "y") {
      // Process vowel team
      if (word[i] != prevChar && !isVowel(prevChar) && prevChar != "y") {
        ++count;
      }
    }
    prevChar = word[i];
  }

  if (count > 1) {
    // Magic e syllable, consonant 'l' must not exist before 'e'
    let lastTwo = word.slice(-2);
    if (lastTwo[0] == "e" && lastTwo[1] != "l") {
      --count;
    }

    // Example: Deactivate
    if (word.includes("ea")) {
      ++count;
    }

    // Example: Worrying
    if (word.includes("ying")) {
      ++count;
    }
  }

  showResult(count);
}
