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

    editor.on('focus', () => removeAllMarker(editor))

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
        let uncompleted_text = editor.session.getValue()
        let answer_text = answer.firstChild.textContent
        editor.setValue(answer_text)
        editor.selection.selectTo(0)

        // Highlight patched
        const diffs = dmp.diff_main(uncompleted_text, answer_text).filter((e) => e['0'] !== -1)
        console.log('diffs:', diffs)

        if (diffs.length >= 0) {
          let cursor = 0
          let text = ''
          diffs.forEach((diff, i) => {
            let [a, b] = Array.from(diff)

            if (i % 2 === 0) {
              cursor += b.length
            } else {
              console.log('---------------')
              console.log('[a, b]:', [a, b])
              let texts = text.split('\n')
              console.log('texts:', texts)

              // let tabs = texts.map((e) => {
              //   console.log('e:', e)
              //   return e.split('\t').length
              // })
              // console.log('tabs:', tabs)

              let row = texts.length - 1

              console.log('b.length:', b.length)

              let prev_text = texts[texts.length - 1].split('\t').join('').length
              console.log('prev_text:', prev_text)

              let prev_tab = texts[texts.length - 1].split('\t').length
              console.log('prev_tab:', prev_tab)

              let newlines = b.split('\n')
              let start_with_new_line = b.indexOf('\n') === 0

              let tab = newlines[0].split('\t').length
              console.log('tab:', tab)

              // newline
              let column = start_with_new_line ? 0 : prev_text
              row = start_with_new_line ? row + 1 : row

              let range = new ace.Range(row, column, row, column + b.length)
              console.log('range:', range)
              editor.session.addMarker(range, 'ace_step', 'line', false)
            }

            text += b
          })
        }
      }

      buttons.push(button)
      answer.remove()
    })

    buttons.forEach((button) => current.append(button))
  })
}
