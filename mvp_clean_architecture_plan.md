I. Core Layer (Entities)
 * Workflow: Represents a workflow with its steps and logic.
   * name: string (e.g., "Incident Report", "Daily Inspection")
   * description: string (optional)
   * steps: array of Step objects
   * Methods:
     * addStep(step: Step)
     * removeStep(stepId: string)
     * getNextStep(currentStepId: string, inputData: any): determines the next step based on current state and input
 * Step: Represents a single step within a workflow.
   * type: enum ('FORM', 'CHECKLIST', 'ROUTINE')
     * ROUTINE: Represents a button that triggers a sub-workflow or a specific action.
   * content:
     * if type is 'FORM': Form object
     * if type is 'CHECKLIST': Checklist object
     * if type is 'ROUTINE': Routine object
   * dependencies: array of Step IDs (steps that must be completed before this one)
 * Form: Represents a form with its fields.
   * name: string
   * fields: array of Field objects
   * Methods:
     * addField(field: Field)
     * removeField(fieldId: string)
 * Field: Represents a single field within a form.
   * type: enum ('TEXT', 'NUMBER', 'DATE', etc.)
   * label: string
 * Checklist: Represents a checklist with its items.
   * name: string
   * items: array of Item objects
   * Methods:
     * addItem(item: Item)
     * removeItem(itemId: string)
 * Item: Represents a single item within a checklist.
   * description: string
 * Routine: Represents a button that triggers an action or sub-workflow.
   * name: string (e.g., "Report Incident", "Start Inspection")
   * action: function (optional, to be executed when the button is clicked)
   * workflow: Workflow object (optional, a sub-workflow to be executed)
II. Application Layer (Use Cases)
 * Workflow Management:
   * CreateWorkflow(name, description, steps)
   * EditWorkflow(workflowId, name, description, steps)
   * DeleteWorkflow(workflowId)
   * GetWorkflow(workflowId)
 * Form Management:
   * CreateForm(name, fields)
   * EditForm(formId, name, fields)
   * DeleteForm(formId)
   * GetForm(formId)
 * Checklist Management:
   * CreateChecklist(name, items)
   * EditChecklist(checklistId, name, items)
   * DeleteChecklist(checklistId)
   * GetChecklist(checklistId)
 * Routine Management:
   * CreateRoutine(name, action, workflow)
   * EditRoutine(routineId, name, action, workflow)
   * DeleteRoutine(routineId)
   * GetRoutine(routineId)
 * Data Management:
   * SaveFormData(formId, data)
   * GetFormData(formId)
III. Interface Adapters (Presentation Layer)
 * Web UI (Mobile-First):
   * Frameworks: React, Vue.js, or similar
   * Controllers:  Handle user interactions, call use cases, and update views.
     * WorkflowBuilder: handles workflow creation and editing.
     * FormBuilder: handles form creation and editing.
     * ChecklistBuilder: handles checklist creation and editing.
     * RoutineBuilder: handles routine creation and editing.
     * WorkflowPlayer: handles displaying and interacting with a workflow.
   * Views:  Render the UI using HTML, CSS, and JavaScript.
     * WorkflowBuilderView: displays the workflow builder interface.
     * FormBuilderView: displays the form builder interface.
     * ChecklistBuilderView: displays the checklist builder interface.
     * RoutineBuilderView: displays the routine builder interface.
     * WorkflowPlayerView: displays the workflow player interface (forms, checklists, routine buttons).
IV. Infrastructure Layer (Frameworks and Drivers)
 * Database:
   * SQLite (for local storage in the browser)
 * Web Framework:
   * React, Vue.js, or similar
Key Considerations:
 * Dependency Injection: Use dependency injection to decouple layers.
 * Interfaces: Define clear interfaces between layers.
 * State Management: Manage the state of the workflow execution (current step, user input) in the WorkflowPlayer component.
 * User Experience:  Design the WorkflowPlayerView to provide a clear and intuitive experience for users as they navigate through the workflow.
This plan focuses on the core features for a local MVP, allowing users to create workflows, add routines, and interact with forms and checklists. It prioritizes a simple and intuitive user experience for building and playing workflows on a mobile device.
