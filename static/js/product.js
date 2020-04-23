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

$("#modal-remove-btn-remove").click(() => {
  $.ajax({
    method: "DELETE",
    url: "",
  }).done(() => {
    window.location.href = "/dashboard"
  })
})
