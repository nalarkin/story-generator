pub struct Sentence {
  pub noun_phrase: NounPhrase,
  pub verb_phrase: VerbPhrase,
  // noun phrase
  // T
  // Noun
  // Verb phrase
  // verb
  // Noun phrase
  // T ?
  // Noun
}
impl Sentence {
  pub fn build(&self) -> String {
    // let arr = [&self.noun_phrase.build(), &self.verb_phrase.build()];
    // arr.join(" ").clone()
    let a = &self.noun_phrase.build();
    let b = &self.verb_phrase.build();
    a.to_string() + b
  }
}

// pub trait Element {
//   fn build(&self) -> String;
// }

pub struct NounPhrase;
impl NounPhrase {
  fn build(&self) -> String {
    String::from("the dog")
  }
}
pub struct VerbPhrase;
impl VerbPhrase {
  fn build(&self) -> String {
    String::from("ran to")
  }
}

trait Nonterminal {
  fn print_terminals(&self) -> String;
}

struct Terminal {
  value: String,
}

impl Terminal {
  pub fn print_terminals(&self) -> String {
    format!("{}", &self.value)
  }
}

pub struct Operator {
  child: Terminal,
}

impl Nonterminal for Operator {
  fn print_terminals(&self) -> String {
    format!("{}", self.child.print_terminals())
  }
}

// clause, sentence 	S → NP VP
// noun phrase 	NP → {(DET)3 (AdjP)n N (PP)n   || → {pro}

// verb phrase 	VP → (AUX)3 V (NP)2 (PP)n (ADV)n (ADJ)n
// prepositional phrase 	PP → P (NP)
// adjective phrase 	AdjP → (DEG) ADJ (PP)
// adverb phrase 	AdvP → (DEG) ADV (PP)
// tense 	TNS → {PAST} ||  → {PRES} ||  → {MOD}
// perfect aspect 	PERF → HAVE + Perfect participle
// progressive aspect 	PROG → BE + Progressive participle
// passive voice 	PASS → BE + Passive participle
// any phrase 	XP → XP CONJ XP, where XP indicates any type of phrase (NP or VP, ADJP, etc.)

// S (sentence) 	a syntactic unit that consists of one or more clauses, contains a subject and a predicate, and expresses a proposition
// NP (noun phrase) 	a phrase composed of a noun as its head and the optional modifiers and determiners of the noun; alternatively may be composed of a single pronoun
// VP (verb phrase) 	a phrase composed of a verb as its head and including all the dependents of the verb, such as direct and indirect objects, adverbials, and subject complements
// AdjP (adjective phrase) 	a phrase composed of an adjective as its head and the optional modifiers of the adjective; serves as a modifier of noun phrases
// AdvP (adverb phrase) 	a phrase composed of an adverb as its head and optional modifiers of the adverb; serves as a modifier of a verb or a clause
// PP (prepositional phrase) 	a phrase composed of a preposition and its required complement, which must be a noun phrase
// N (noun) 	any member of a class of words refer to people, objects, concepts, and ideas; can be inflected number, gender, and/or case; serves as the subject of a clause or an object of a verb or preposition
// V (verb) 	a member of the word class that expresses actions or events; can be inflected for tense, aspect, mood, etc. functions as the main word in the predicate of a clause
// DET (determiner) 	a word (such as an article, possessive, demonstrative, or quantifier) that expresses the reference, including specificity and quantity, of a noun phrase
// pro (pronoun) 	a member of the functional word class that functions like a noun and can substitute for a noun phrase in a clause
// ADJ (adjective) 	a member of the word class whose function is to specify the properties and attributes of a noun it modifies
// ADV (adverb) 	a member of the word class whose function is to modify such properties of verbs and clauses as time, place, manner, attitude, purpose, etc
// DEG (degree word) 	typically, an adverb whose function is to modify the head of an adjective or adverb phrase; “very” is a canonical degree word.
// P (preposition) 	a word that precedes noun phrases and expresses the relationship between this noun phrase and another element of the clause
// CONJ (conjunction) 	a word that syntactically links words or larger constituents and expresses a semantic relationship between them

// COOR (Coordinating conjunction) - links words or larger constituents of equal syntactic value
// SUBORD (Subordinating conjunction) - links clauses in such a manner that one of them becomes a constituent of another
// COMP (complementizer) 	a conjunction that marks a complement or subordinate clause
// TNS (Tense) 	a verbal category that deictically refers to the time of the action or event expressed in the verb
// PERF (Perfect Aspect) 	a verbal category, and indicating that the action or event expressed by the said verb occurs prior to a specific point in time
// PROG (Progressive Aspect) 	a verbal category that indicates an action or event in progress at a specific point in time
// PASS (Passive Voice) 	a verbal category that indicates that the subject of the marked verb is the recipient or patient of the action rather than its agent
// AUX (Auxiliary (verb)) 	a functional verbal category that accompanies a lexical verb and expresses grammatical distinctions not carried by the said verb, such as tense, aspect, person, number, mood, etc
