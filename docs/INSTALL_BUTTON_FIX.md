# Install Modpack Button - Navigation Fix

## 🐛 Problem

When clicking "Install Modpack" from homepage:

1. User saw Step 1 (Welcome page) briefly before Step 2
2. Clicking back at Step 2 went to Step 1 instead of homepage
3. Extra page appeared in the flow

## ✅ Solution

### 1. Fixed App.svelte Rendering Logic

**Before:**

```svelte
{#if !$onboardingCompleted}
  <OnboardingFlow />
{:else if $currentPage === 'homepage'}
```

**After:**

```svelte
{#if !$onboardingCompleted || $currentPage === 'onboarding'}
  <OnboardingFlow />
{:else if $currentPage === 'homepage'}
```

**Why:** This allows showing onboarding even when it's been completed before, by checking `currentPage === 'onboarding'`.

### 2. Updated startInstallFlow() Function

**Before:**

```javascript
function startInstallFlow() {
  resetOnboarding(); // Cleared data
  onboardingCompleted.set(false); // Changed completed state
  onboardingStartStep.set(2);
  hideUpdateButton.set(true);
  currentPage.set('onboarding');
}
```

**After:**

```javascript
function startInstallFlow() {
  // Don't change onboardingCompleted, just navigate
  onboardingStartStep.set(2); // Start at step 2
  hideUpdateButton.set(true); // Hide update button
  currentPage.set('onboarding');
}
```

**Why:** We don't need to reset onboarding data or change completion state, just navigate.

### 3. Fixed Step Initialization

**Added reactive sync:**

```javascript
// Immediately sync with store before rendering
$: if ($onboardingStartStep && !hasInitialized) {
  currentStep = $onboardingStartStep;
}

onMount(() => {
  currentStep = $onboardingStartStep;
  hasInitialized = true;
});
```

**Why:** The reactive statement sets `currentStep` immediately before render, preventing flash of Step 1.

### 4. Fixed Back Button Logic

```javascript
function previousStep() {
  // If at step 2 and came from homepage, go back to homepage
  if (currentStep === 2 && $onboardingStartStep === 2) {
    onboardingStartStep.set(1);
    hideUpdateButton.set(false);
    // Restore onboarding completed state if it was completed before
    const wasCompleted = localStorage.getItem('naha-onboarding-completed');
    if (wasCompleted) {
      onboardingCompleted.set(true);
    }
    currentPage.set('homepage');
    return;
  }
  
  if (currentStep > 1) {
    currentStep--;
  }
}
```

**Why:** When clicking back from step 2 (if came from homepage), it:

- Resets control variables
- Restores onboardingCompleted state from localStorage
- Navigates back to homepage

## 🎯 Flow Diagram

### From Homepage Install Modpack

```text
Homepage
   ↓ [Click 🎮 Install Modpack]
   ↓ (sets onboardingStartStep=2, hideUpdateButton=true)
   ↓
Step 2: Minecraft Check
   ├─ onboardingStartStep = 2 ✓
   ├─ hideUpdateButton = true ✓
   └─ currentStep immediately set to 2 ✓
   
   ↓ [Click ← Back]
   ↓ (detects: currentStep=2 AND onboardingStartStep=2)
   ↓ (restores onboardingCompleted from localStorage)
   ↓
Homepage ✓
```

### From Homepage Take me through setup

```text
Homepage
   ↓ [Click 🚀 Take me through setup]
   ↓ (resets onboarding, sets onboardingStartStep=1)
   ↓
Step 1: Welcome
   ↓ [Click Get Started]
   ↓
Step 2: Minecraft Check
   ├─ onboardingStartStep = 1
   ├─ hideUpdateButton = false
   └─ Shows all 3 buttons (triforce)
   
   ↓ [Click ← Back]
   ↓ (detects: currentStep=2 BUT onboardingStartStep=1)
   ↓
Step 1: Welcome ✓
```

## 🔍 Debug Logging

Added console logs to track the flow:

- `🎮 [HOMEPAGE]` - Homepage button clicks
- `🔧 [FLOW]` - Onboarding flow events
- Shows: currentStep, startStep, navigation decisions

## ✅ Testing Checklist

1. **Install Modpack from Homepage:**

   - [ ] Click "🎮 Install Modpack"
   - [ ] Should go directly to Step 2 (Minecraft Check)
   - [ ] Should NOT see Step 1 (Welcome)
   - [ ] Should only see 2 buttons (✅ Yes, 📥 No)
   - [ ] Update button (🔄) should be hidden
   - [ ] Click "← Back"
   - [ ] Should return to Homepage
   - [ ] Should NOT see Step 1

2. **Take me through setup:**

   - [ ] Click "🚀 Take me through the setup"
   - [ ] Should see Step 1 (Welcome)
   - [ ] Click "Get Started"
   - [ ] Should see Step 2 (Minecraft Check)
   - [ ] Should see all 3 buttons (triforce)
   - [ ] Click "← Back"
   - [ ] Should go to Step 1 (Welcome)

## 📝 Changes Summary

| File | Change |
|------|--------|
| `App.svelte` | Updated rendering condition to check `currentPage === 'onboarding'` |
| `HomePage.svelte` | Removed `resetOnboarding()` and `onboardingCompleted.set(false)` from install flow |
| `OnboardingFlow.svelte` | Added reactive sync, improved back button, restore state on back |

## 🎉 Result

- ✅ No extra pages
- ✅ Starts directly at Step 2
- ✅ Back button goes to homepage
- ✅ onboardingCompleted state preserved
- ✅ Triforce button layout
- ✅ Update button hidden when from homepage

---

**Status:** Fixed and Ready to Test 🚀
