describe("Link", () => {
  it("should have provided text", () => {
    cy
      .visit('/')
      .get('[data-test="link"]')
  })
})