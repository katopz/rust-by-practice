window.onload = function () {
  // Assume edit mode
  if (window.location.hostname !== 'localhost' && window.location.hostname !== '[::1]') return

  function getRoWColumnRange(range) {
    const { start, end } = range
    if (start.row === end.row) {
      return { row: Math.min(start.row, end.row), column: Math.min(start.column, end.column) }
    } else if (start.row < end.row) {
      return start
    } else {
      return end
    }
  }

  function removeAllMarker(editor) {
    const prevMarkers = editor.session.getMarkers()
    if (prevMarkers) {
      const prevMarkersArr = Object.keys(prevMarkers)
      for (let item of prevMarkersArr) {
        editor.session.removeMarker(prevMarkers[item].id)
      }
    }
  }

  window.editors.forEach((editor, i) => {
    editor.raw = editor.session.getValue()
    editor.type = editor.session.getValue().indexOf('__') > 0 ? 'under' : 'at'
    editor.solutions = []
    editor.removes = []
    editor.onCopy = function () {
      const { start, end } = this.getSelectionRange()
      const a = { row: Math.min(start.row, end.row), column: Math.min(start.column, end.column) }
      const b = { row: Math.max(start.row, end.row), column: Math.max(start.column, end.column) }
      editor.removes.push({ start: a, end: b })

      editor.type = 'replace'
    }

    editor.on('focus', () => removeAllMarker(editor))

    editor.onCut = function () {
      const range = this.getSelectionRange()
      const { row, column } = getRoWColumnRange(range)
      const raw_length = editor.session.getValue().length

      const raw_copy_text = this.getCopyText()
      const copy_text = raw_copy_text.replace(/`/g, '\\`')

      // all
      if (raw_length === raw_copy_text.length) {
        editor.type = 'all'
      }

      // all | at | under | replace
      let solution = '`' + copy_text + '`'
      let fn = 'solveAll'
      switch (editor.type) {
        case 'at':
          this.solutions.push(JSON.stringify([row, column, copy_text]))
          solution = `[${this.solutions}]`
          fn = 'solveAt'
          break
        case 'under':
          this.solutions.push(JSON.stringify(copy_text))
          solution = `[${this.solutions}]`
          fn = 'solveUnder'
          break
        case 'replace':
          this.solutions.push(JSON.stringify([row, column, copy_text, editor.removes]))
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
      removeAllMarker(editor)

      // insert
      arguments.forEach((answers) => {
        const [row, column, answer] = answers
        editor.session.insert({ row, column }, answer)
        editor.session.addMarker(new ace.Range(row, column, row, column + answer.length), 'ace_step', 'line', false)

        // More highlight for newline
        const newlines = answer.split('\n')
        let j = 0
        let k = 0
        while (newlines[j++] === '') k++
        if (k > 0) {
          editor.session.addMarker(new ace.Range(row + k, 0, row + k, 0 + answer.length), 'ace_step', 'line', false)
        }
      })
    }

    e.solveReplace = (...arguments) => {
      // restore
      editor.session.setValue(editor.raw)
      removeAllMarker(editor)

      // insert
      arguments.forEach((answers, i) => {
        const [row, column, answer, removes] = answers
        editor.session.replace(removes[i], '')
        editor.session.insert({ row, column }, answer)
        editor.session.addMarker(new ace.Range(row, column, row, column + answer.length), 'ace_step', 'line', false)

        // More highlight for newline
        const newlines = answer.split('\n')
        let j = 0
        let k = 0
        while (newlines[j++] === '') k++
        if (k > 0) {
          editor.session.addMarker(new ace.Range(row + k, 0, row + k, 0 + answer.length), 'ace_step', 'line', false)
        }
      })
    }

    e.solveAll = (answer) => {
      editor.session.setValue(answer)
    }
  })
}
