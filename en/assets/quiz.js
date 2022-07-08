window.onload = function () {
  // Assume edit mode
  if (window.location.hostname !== 'localhost' && window.location.hostname !== '[::1]') return

  window.editors.forEach((editor, i) => {
    editor.raw = editor.session.getValue()
    editor.type = editor.session.getValue().indexOf('__') > 0 ? 'under' : 'at'
    editor.solutions = []
    // editor.replaces = []
    // editor.onCopy = function () {
    //   const { start, end } = this.getSelectionRange()
    //   const a = { row: Math.min(start.row, end.row), column: Math.min(start.column, end.column) }
    //   const b = { row: Math.max(start.row, end.row), column: Math.max(start.column, end.column) }
    //   editor.replaces.push({ start: a, end: b })

    //   console.log(editor.replaces)
    //   editor.type = 'replace'
    // }

    editor.onCut = function () {
      const { start, end } = this.getSelectionRange()
      const { row, column } = { row: Math.min(start.row, end.row), column: Math.min(start.column, end.column) }

      const text = this.getCopyText().replace(/`/g, '\\`')

      // all
      if (row === 0 && column === 0) {
        editor.type = 'all'
      }

      // all | at | under | replace
      let solution = '`' + text + '`'
      let fn = 'solveAll'
      switch (editor.type) {
        case 'at':
          this.solutions.push(JSON.stringify([row, column, text]))
          solution = `[${this.solutions}]`
          fn = 'solveAt'
          break
        case 'under':
          this.solutions.push(JSON.stringify(text))
          solution = `[${this.solutions}]`
          fn = 'solveUnder'
          break
        case 'replace':
          this.solutions.push(JSON.stringify([row, column, text]))
          solution = `[${this.solutions}]`
          fn = 'solveReplace'
          break
      }

      // Copy
      navigator.clipboard.writeText(`<script>let answers_${i + 1} = ${solution}</script>
<button class="hint" id="hint_${i + 1}" onclick="this.${fn}(${editor.type === 'all' ? '' : '...'}answers_${i + 1})">💡 HINT</button>`)

      // super
      this.commands.exec('cut', this)
    }.bind(editor)
  })
  ;[...document.querySelectorAll('.hint')].forEach((e) => {
    let editor = window.editors[parseInt(e.id.split('_')[1]) - 1]

    e.solveUnder = (...arguments) => {
      // restore
      editor.session.setValue(editor.raw)

      // get __ position
      editor.findAll('__')
      const ranges = editor.getSelection().getAllRanges()
      ranges.forEach((range, i) => {
        const { start, end } = range
        const { row, column } = { row: Math.min(start.row, end.row), column: Math.min(start.column, end.column) }
        const answer = arguments[i]
        editor.session.addMarker(new ace.Range(row, column, row, column + answer.length), 'ace_step', 'line', false)
      })

      editor.session.setValue(editor.session.getValue().replace(/(__)/g, () => arguments.shift()))
    }

    e.solveAt = (...arguments) => {
      // restore
      editor.session.setValue(editor.raw)

      // insert
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
