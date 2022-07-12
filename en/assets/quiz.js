const get_answers = (current) => {
  let _current = current
  let answers = []
  while (_current.nextSibling && _current.nextSibling.nextSibling && _current.nextSibling.nextSibling.nodeName === 'PRE' && _current.nextSibling.nextSibling.firstChild.childNodes[1].className.indexOf('answer')) {
    answers.push((_current = _current.nextSibling.nextSibling))
  }

  return answers
}

window.onload = function () {
  const dmp = new diff_match_patch()

  window.editors.forEach((editor, i) => {
    // keep raw
    editor.raw = editor.session.getValue()

    // Clear marker on focus
    function removeAllMarker(editor) {
      const prevMarkers = editor.session.getMarkers()
      if (prevMarkers) {
        const prevMarkersArr = Object.keys(prevMarkers)
        for (let item of prevMarkersArr) {
          editor.session.removeMarker(prevMarkers[item].id)
        }
      }
    }

    editor.on('change', () => removeAllMarker(editor))

    // Find answers after editor
    let current = editor.container.parentElement.parentNode
    editor.answers = get_answers(current)

    // Create buttons from answers
    let buttons = []
    editor.answers.forEach((answer, i) => {
      let button = document.createElement('button')
      let button_text = `🦀 HINT` + (editor.answers.length === 1 ? `` : ` ${i + 1}`)
      button.append(button_text)
      button.className = 'hint'
      button.onclick = () => {
        let uncompleted_text = editor.raw
        let answer_text = answer.firstChild.textContent
        editor.setValue(answer_text)
        editor.selection.selectTo(0)

        // Highlight patched
        const diffs = dmp.diff_main(uncompleted_text, answer_text).filter((e) => e['0'] !== -1)
        // console.log('diffs:', diffs)

        if (diffs.length >= 0) {
          let text = ''

          diffs.forEach((diff, i) => {
            let patch_type = diff['0']
            let current_text = diff['1']

            // console.log('patch_type:', patch_type)
            if (i % 2 === 0 || (patch_type === 0 && i === diffs.length - 1)) {
              // Do nothing
            } else {
              let texts = text.split('\n')

              let row = texts.length - 1
              let prev_text_size = texts[texts.length - 1].split('\t').join('').length

              let newlines = current_text.split('\n')

              let chunks = newlines.map((e) => ({
                current_text: e,
                is_blank_line: e === '',
                tab_size: e.split('\t').length * 4
              }))

              chunks.forEach((e, i) => {
                // console.log(e)
                // newline
                const is_prev_blank_line = i > 0 && chunks[i - 1].current_text === ''
                let start_with_new_line = e.current_text.indexOf('\n') === 0 || is_prev_blank_line
                // console.log('start_with_new_line:', start_with_new_line)
                if (is_prev_blank_line) {
                  row = row - 1
                }

                // console.log('e.tab_size:', e.tab_size)
                // console.log('prev_text_size:', prev_text_size)

                let column = start_with_new_line ? e.tab_size : prev_text_size
                row = start_with_new_line ? row + 1 : row

                // console.log('current_text:', e.current_text)
                let current_text_no_tab = e.current_text.split('\t').join('').split('    ').join('').length
                // console.log('current_text_no_tab:', current_text_no_tab)

                let range = new ace.Range(row, column, row, column + current_text_no_tab)
                // console.log('range:', range)

                editor.session.addMarker(range, 'ace_step', 'line', false)

                row = row + 1
              })
            }

            text += current_text
          })
        }
      }

      buttons.push(button)
      answer.remove()
    })

    buttons.forEach((button) => current.append(button))
  })
}
