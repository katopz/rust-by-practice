window.onload = function () {
  // Assume edit mode
  if (window.location.hostname !== 'localhost') return

  window.editors.forEach((editor) => {
    editor.solutions = []
    editor.onCut = function () {
      const { row, column } = this.selection.getSelectionAnchor()
      const text = this.getCopyText()

      // solveAt
      this.solutions.push(JSON.stringify([row, column, btoa(text)]))
      const buttonText = `<button class="hint" onclick='this.solveAt(${this.solutions})'>💡 HINT</button>`

      // Copy
      navigator.clipboard.writeText(buttonText)

      // super
      this.commands.exec('cut', this)
    }.bind(editor)
  })
  ;[...document.querySelectorAll('.hint')].forEach((e, i) => {
    e.id = `hint_${i}`
    let editor = window.editors[i]

    e.solve = (answers) => {
      editor.session.setValue(editor.session.getValue().replace(/(__)/g, () => answers.shift()))
    }

    e.solveAt = (...arguments) => {
      arguments.forEach((answers) => {
        let row = answers[0]
        let column = answers[1]
        let answer = atob(answers[2])

        editor.session.insert({ row, column }, answer)
      })
    }

    e.solveAll = (answer) => {
      editor.session.setValue(answer)
    }
  })
}
