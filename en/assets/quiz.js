window.onload = function () {
  // Assume edit mode
  if (window.location.hostname !== 'localhost') return

  window.editors.forEach((editor, i) => {
    editor.type = editor.session.getValue().indexOf('__') > 0 ? 'under' : 'at'
    editor.solutions = []
    editor.onCut = function () {
      const { row, column } = this.selection.getSelectionAnchor()
      const text = this.getCopyText()

      // at | under
      if (editor.type === 'at') {
        this.solutions.push(JSON.stringify([row, column, text]))
      } else {
        this.solutions.push(JSON.stringify(text))
      }

      const fn = editor.type === 'at' ? 'solveAt' : 'solveUnder'

      // Copy
      navigator.clipboard.writeText(`<script>let answers_${i} = [${this.solutions}]</script>
<button class="hint" onclick="this.${fn}(...answers_${i})">💡 HINT</button>`)

      // super
      this.commands.exec('cut', this)
    }.bind(editor)
  })
  ;[...document.querySelectorAll('.hint')].forEach((e, i) => {
    e.id = `hint_${i}`
    let editor = window.editors[i]

    e.solveUnder = (...arguments) => {
      editor.findAll('__')
      const ranges = editor.getSelection().getAllRanges()
      ranges.forEach((range, i) => {
        const { row, column } = range.cursor
        const answer = arguments[i]
        editor.session.addMarker(new ace.Range(row, column - 2, row, column - 2 + answer.length), 'ace_step', 'line', false)
      })

      editor.session.setValue(editor.session.getValue().replace(/(__)/g, () => arguments.shift()))
    }

    e.solveAt = (...arguments) => {
      arguments.forEach((answers) => {
        const [row, column, answer] = answers
        editor.session.insert({ row, column }, answer)
        editor.session.addMarker(new ace.Range(row, column, row, column + answer.length), 'ace_step', 'line', false)
      })
    }

    e.solveAll = (answer) => {
      editor.session.setValue(answer)
    }
  })
}
