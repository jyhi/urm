$("#modal-edit-btn-add-field").click(() => {
  const n = $("#modal-edit-body .field").length;

  $("#modal-edit-body").append(`
<div class="row no-gutters mt-3 field" id="modal-edit-field${n + 1}">
  <div class="col">
    <label for="modal-edit-input-field${n + 1}-key" class="sr-only">Key</label>
    <input class="form-control key" id="modal-edit-input-field${n + 1}-key" placeholder="key" required>
    <div class="my-2"></div>
    <label for="modal-edit-input-field${n + 1}-value" class="sr-only">Value</label>
    <input class="form-control value" id="modal-edit-input-field${n + 1}-value" placeholder="value" required>
  </div>
  <div class="col-auto ml-2 d-flex align-items-stretch">
    <button class="btn btn-block btn-outline-danger" id="modal-edit-btn-remove-field${n + 1}" aria-label="Remove this field" onclick="removeField('#modal-edit-field${n + 1}')">&times;</button>
  </div>
</div>`)
})

const removeField = (f) => {
  $(f).detach();
}

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

  const keys = $.map($("#modal-edit-body .real-key"), (v) => v.textContent)
    .concat($.map($("#modal-edit-body .key"), (v) => v.value))
  const values = $.map($("#modal-edit-body .value"), (v) => v.value)
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
