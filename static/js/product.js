$("#modal-change-btn-submit").click(() => {
  if (!$("#modal-change-input-amount")[0].validity.valid)
    return

  $.ajax({
    method: "PATCH",
    url: "",
    contentType: "application/json",
    data: JSON.stringify({amount: $("#modal-change-input-amount").val()})
  }).done(() => {
    $("#modal-change").modal("hide")
    location.reload()
  })
})

$("#modal-edit-btn-submit").click(() => {
  const validity = $
    .map($("#modal-edit-body input"), (v) => v.validity.valid)
    .reduce((a, v) => a & v, true)
  if (!validity)
    return

  const keys = $.map($("#modal-edit-body label"), (v) => v.textContent)
  const values = $.map($("#modal-edit-body input"), (v) => v.value)
  const object = Object.fromEntries(keys.map((_, i) => [keys[i], values[i]]))

  $.ajax({
    method: "PUT",
    url: "",
    contentType: "application/json",
    data: JSON.stringify(object)
  }).done(() => {
    $("#modal-edit").modal("hide")
    location.reload()
  })
})

$("#modal-remove-btn-remove").click(() => {
  $.ajax({
    method: "DELETE",
    url: "",
  }).done(() => {
    window.location.href = "/dashboard"
  })
})
