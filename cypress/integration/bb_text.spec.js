describe("BBText", () => {
  it("render normal text by default", () => {
    cy
      .visit("/")
      .get("p[data-test='normal-paragraph']")
      .should('contain', 'paragraph text')
  })

  it("renders a title", () => {
    cy
      .visit("/")
      .get("h1[data-test='title-text']")
      .should("contain", "title text")
  })
})